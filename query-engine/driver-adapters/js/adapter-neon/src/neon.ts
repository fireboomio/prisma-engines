import type neon from '@neondatabase/serverless'
import { Debug, ok, err } from '@prisma/driver-adapter-utils'
import type {
  DriverAdapter,
  ResultSet,
  Query,
  Queryable,
  Transaction,
  Result,
  TransactionOptions,
} from '@prisma/driver-adapter-utils'
import { fieldToColumnType } from './conversion'

const debug = Debug('prisma:driver-adapter:neon')

type ARRAY_MODE_ENABLED = true

type PerformIOResult = neon.QueryResult<any> | neon.FullQueryResults<ARRAY_MODE_ENABLED>

/**
 * Base class for http client, ws client and ws transaction
 */
abstract class NeonQueryable implements Queryable {
  readonly flavour = 'postgres'

  async queryRaw(query: Query): Promise<Result<ResultSet>> {
    const tag = '[js::query_raw]'
    debug(`${tag} %O`, query)

    const { fields, rows } = await this.performIO(query)

    const columns = fields.map((field) => field.name)
    const resultSet: ResultSet = {
      columnNames: columns,
      columnTypes: fields.map((field) => fieldToColumnType(field.dataTypeID)),
      rows,
    }

    return ok(resultSet)
  }

  async executeRaw(query: Query): Promise<Result<number>> {
    const tag = '[js::execute_raw]'
    debug(`${tag} %O`, query)

    const { rowCount: rowsAffected } = await this.performIO(query)

    // Note: `rowsAffected` can sometimes be null (e.g., when executing `"BEGIN"`)
    return ok(rowsAffected ?? 0)
  }

  abstract performIO(query: Query): Promise<PerformIOResult>
}

/**
 * Base class for WS-based queryables: top-level client and transaction
 */
class NeonWsQueryable<ClientT extends neon.Pool | neon.PoolClient> extends NeonQueryable {
  constructor(protected client: ClientT) {
    super()
  }

  override async performIO(query: Query): Promise<PerformIOResult> {
    const { sql, args: values } = query

    try {
      return await this.client.query({ text: sql, values, rowMode: 'array' })
    } catch (e) {
      const error = e as Error
      debug('Error in performIO: %O', error)
      throw error
    }
  }
}

class NeonTransaction extends NeonWsQueryable<neon.PoolClient> implements Transaction {
  constructor(client: neon.PoolClient, readonly options: TransactionOptions) {
    super(client)
  }

  async commit(): Promise<Result<void>> {
    debug(`[js::commit]`)

    this.client.release()
    return Promise.resolve(ok(undefined))
  }

  async rollback(): Promise<Result<void>> {
    debug(`[js::rollback]`)

    this.client.release()
    return Promise.resolve(ok(undefined))
  }
}

export class PrismaNeon extends NeonWsQueryable<neon.Pool> implements DriverAdapter {
  private isRunning = true

  constructor(pool: neon.Pool) {
    super(pool)
  }

  async startTransaction(): Promise<Result<Transaction>> {
    const options: TransactionOptions = {
      usePhantomQuery: false,
    }

    const tag = '[js::startTransaction]'
    debug(`${tag} options: %O`, options)

    const connection = await this.client.connect()
    return ok(new NeonTransaction(connection, options))
  }

  async close() {
    if (this.isRunning) {
      await this.client.end()
      this.isRunning = false
    }
    return ok(undefined)
  }
}

export class PrismaNeonHTTP extends NeonQueryable implements DriverAdapter {
  constructor(private client: neon.NeonQueryFunction<any, any>) {
    super()
  }

  override async performIO(query: Query): Promise<PerformIOResult> {
    const { sql, args: values } = query
    return await this.client(sql, values, {
      arrayMode: true,
      fullResults: true,
    })
  }

  startTransaction(): Promise<Result<Transaction>> {
    return Promise.reject(new Error('Transactions are not supported in HTTP mode'))
  }

  async close() {
    return ok(undefined)
  }
}
