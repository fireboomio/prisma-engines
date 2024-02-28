use query_engine_tests::*;

#[test_suite(only(Postgres))]
mod postgres {
    use indoc::indoc;
    use query_engine_tests::run_query;

    fn schema_int() -> String {
        let schema = indoc! {
            r#"model Model {
              #id(id, String, @id, @default(cuid()))
              int      Int    @test.Integer
              sInt     Int    @test.SmallInt
              bInt     BigInt @test.BigInt
              oid      Int    @test.Oid
              inc_int  Int    @test.Integer     @default(autoincrement())
              inc_sInt Int    @test.SmallInt    @default(autoincrement())
              inc_bInt BigInt @test.BigInt      @default(autoincrement())
            }"#
        };

        schema.to_owned()
    }

    //"Postgres native int types" should "work"
    #[connector_test(schema(schema_int))]
    async fn native_int_types(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
            createOneModel(
              data: {
                int: 2147483647
                sInt: 32767
                bInt: "9223372036854775807"
                oid: 0
              }
            ) {
              int
              sInt
              bInt
              oid
              inc_int
              inc_sInt
              inc_bInt
            }
          }"#),
          @r###"{"data":{"createOneModel":{"int":2147483647,"sInt":32767,"bInt":"9223372036854775807","oid":0,"inc_int":1,"inc_sInt":1,"inc_bInt":"1"}}}"###
        );

        Ok(())
    }

    fn schema_decimal() -> String {
        let schema = indoc! {
            r#"model Model {
              #id(id, String, @id, @default(cuid()))
              float    Float   @test.Real
              dfloat   Float   @test.DoublePrecision
              decFloat Decimal @test.Decimal(2, 1)
              money    Decimal @test.Money
            }"#
        };

        schema.to_owned()
    }

    // "Postgres native decimal types" should "work"
    #[connector_test(schema(schema_decimal), only(Postgres), exclude(CockroachDb))]
    async fn native_decimal_types(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
            createOneModel(
              data: {
                float: 1.1
                dfloat: 2.2
                decFloat: 3.1234
                money: 3.51
              }
            ) {
              float
              dfloat
              decFloat
              money
            }
          }"#),
          // decFloat is cut due to precision
          @r###"{"data":{"createOneModel":{"float":1.1,"dfloat":2.2,"decFloat":"3.1","money":"3.51"}}}"###
        );

        Ok(())
    }

    fn schema_decimal_cockroach() -> String {
        let schema = indoc! {
            r#"model Model {
              #id(id, String, @id, @default(cuid()))
              float    Float   @test.Float4
              dfloat   Float   @test.Float8
              decFloat Decimal @test.Decimal(2, 1)
            }"#
        };

        schema.to_owned()
    }

    // Cockroach does not support money.
    #[connector_test(schema(schema_decimal_cockroach), only(CockroachDb))]
    async fn native_decimal_types_cockroach(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
            createOneModel(
              data: {
                float: 1.1
                dfloat: 2.2
                decFloat: 3.1234
              }
            ) {
              float
              dfloat
              decFloat
            }
          }"#),
          // decFloat is cut due to precision
          @r###"{"data":{"createOneModel":{"float":1.1,"dfloat":2.2,"decFloat":"3.1"}}}"###
        );

        Ok(())
    }

    fn schema_string() -> String {
        let schema = indoc! {
            r#"model Model {
              #id(id, String, @id, @default(cuid()))
              char  String @test.Char(10)
              vChar String @test.VarChar(11)
              text  String @test.Text
              bit   String @test.Bit(4)
              vBit  String @test.VarBit(5)
              uuid  String @test.Uuid
              ip    String @test.Inet
            }"#
        };

        schema.to_owned()
    }

    // "Postgres native string types" should "work"
    #[connector_test(schema(schema_string))]
    async fn native_string(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
            createOneModel(
              data: {
                char: "1234567890"
                vChar: "12345678910"
                text: "text"
                bit: "1010"
                vBit: "00110"
                uuid: "123e4567-e89b-12d3-a456-426614174000"
                ip: "127.0.0.1"
              }
            ) {
              char
              vChar
              text
              bit
              vBit
              uuid
              ip
            }
          }"#),
          @r###"{"data":{"createOneModel":{"char":"1234567890","vChar":"12345678910","text":"text","bit":"1010","vBit":"00110","uuid":"123e4567-e89b-12d3-a456-426614174000","ip":"127.0.0.1"}}}"###
        );

        Ok(())
    }

    fn schema_other_types() -> String {
        let schema = indoc! {
            r#"model Model {
              #id(id, String, @id, @default(cuid()))
              bool  Boolean @test.Boolean
              byteA Bytes   @test.ByteA
              xml   String  @test.Xml
              json  Json    @test.Json
              jsonb Json    @test.JsonB
            }"#
        };

        schema.to_owned()
    }

    // "Other Postgres native types" should "work"
    #[connector_test(schema(schema_other_types), only(Postgres), exclude(CockroachDb))]
    async fn native_other_types(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
            createOneModel(
              data: {
                bool: true
                byteA: "dGVzdA=="
                xml: "<wurst>salat</wurst>"
                json: "{}"
                jsonb: "{\"a\": \"b\"}"
              }
            ) {
              bool
              byteA
              xml
              json
              jsonb
            }
          }"#),
          @r###"{"data":{"createOneModel":{"bool":true,"byteA":"dGVzdA==","xml":"<wurst>salat</wurst>","json":"{}","jsonb":"{\"a\":\"b\"}"}}}"###
        );

        Ok(())
    }

    fn schema_other_types_cockroach() -> String {
        let schema = indoc! {
            r#"model Model {
              #id(id, String, @id, @default(cuid()))
              bool  Boolean @test.Bool
              byteA Bytes   @test.Bytes
              jsonb Json    @test.JsonB
            }"#
        };

        schema.to_owned()
    }

    // Cockroach does not support XML.
    #[connector_test(schema(schema_other_types_cockroach), only(CockroachDb))]
    async fn native_other_types_cockroach(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
            createOneModel(
              data: {
                bool: true
                byteA: "dGVzdA=="
                jsonb: "{\"a\": \"b\"}"
              }
            ) {
              bool
              byteA
              jsonb
            }
          }"#),
          @r###"{"data":{"createOneModel":{"bool":true,"byteA":"dGVzdA==","jsonb":"{\"a\":\"b\"}"}}}"###
        );

        Ok(())
    }

    fn schema_date() -> String {
        let schema = indoc! {
            r#"model Model {
              #id(id, String, @id, @default(cuid()))
              date       DateTime @test.Date
              date_2     DateTime @test.Date
              time       DateTime @test.Time(3)
              time_2     DateTime @test.Time(3)
              time_tz    DateTime @test.Timetz(3)
              time_tz_2  DateTime @test.Timetz(3)
              ts         DateTime @test.Timestamp(3)
              ts_2       DateTime @test.Timestamp(3)
              ts_tz      DateTime @test.Timestamptz(3)
              ts_tz_2    DateTime @test.Timestamptz(3)
            }"#
        };

        schema.to_owned()
    }

    // "Postgres native date types" should "work"
    #[connector_test(schema(schema_date))]
    async fn native_date(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
            createOneModel(
              data: {
                date: "2016-09-24T00:00:00.000Z"
                date_2: "2016-09-24T00:00:00.000+03:00"
                time: "1111-11-11T13:02:20.321Z"
                time_2: "1111-11-11T13:02:20.321+03:00"
                time_tz: "1111-11-11T13:02:20.321Z"
                time_tz_2: "1111-11-11T13:02:20.321+03:00"
                ts: "2016-09-24T14:01:30.213Z"
                ts_2: "2016-09-24T14:01:30.213+03:00"
                ts_tz: "2016-09-24T14:01:30.213Z"
                ts_tz_2: "2016-09-24T14:01:30.213+03:00"
              }
            ) {
              date
              date_2
              time
              time_2
              time_tz
              time_tz_2
              ts
              ts_2
              ts_tz
              ts_tz_2
            }
          }"#),
          @r###"{"data":{"createOneModel":{"date":"2016-09-24T00:00:00.000Z","date_2":"2016-09-23T00:00:00.000Z","time":"1970-01-01T13:02:20.321Z","time_2":"1970-01-01T10:02:20.321Z","time_tz":"1970-01-01T13:02:20.321Z","time_tz_2":"1970-01-01T10:02:20.321Z","ts":"2016-09-24T14:01:30.213Z","ts_2":"2016-09-24T11:01:30.213Z","ts_tz":"2016-09-24T14:01:30.213Z","ts_tz_2":"2016-09-24T11:01:30.213Z"}}}"###
        );

        Ok(())
    }

    fn schema_native_fixed_size_char() -> String {
        let schema = indoc! {
            r#"model ModelA {
              #id(id, String, @id, @test.Char(16))
              b_id String? @unique @test.Char(16)
              b    ModelB? @relation(fields: [b_id], references: [id])
            }

            model ModelB {
              #id(id, String, @id, @test.Char(16))
              a  ModelA?
            }"#
        };

        schema.to_owned()
    }

    // "Postgres native fixed-size char type" should "be handled correctly wrt. padding for comparisons"
    #[connector_test(schema(schema_native_fixed_size_char))]
    async fn native_fixed_size_char(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
            createOneModelA(data: {
              id: "1234"
               b: { create: { id: "4321" } }
            }) {
              id
              b { id }
            }
          }"#),
          // This is correct - postgres returns padded strings (as opposed to MySQL for example, where it's trimmed).
          @r###"{"data":{"createOneModelA":{"id":"1234            ","b":{"id":"4321            "}}}}"###
        );

        Ok(())
    }

    fn schema_ewkt_geometry() -> String {
        let schema = indoc! {
            r#"model Model {
            @@schema("test")
            #id(id, String, @id, @default(cuid()))
            geometry             Geometry @test.Geometry(Geometry)
            geometry_point       Geometry @test.Geometry(Point)
            geometry_line        Geometry @test.Geometry(LineString)
            geometry_poly        Geometry @test.Geometry(Polygon)
            geometry_multipoint  Geometry @test.Geometry(MultiPoint)
            geometry_multiline   Geometry @test.Geometry(MultiLineString)
            geometry_multipoly   Geometry @test.Geometry(MultiPolygon)
            geometry_collection  Geometry @test.Geometry(GeometryCollection)
          }"#
        };

        schema.to_owned()
    }

    // "PostGIS common geometry types" should "work"
    #[connector_test(
        only(Postgres("15-postgis"), CockroachDb),
        schema(schema_ewkt_geometry),
        db_schemas("public", "test")
    )]
    async fn native_ewkt_geometry(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
          createOneModel(
            data: {
              geometry: "POINT(1 2)"
              geometry_point: "POINT(1 2)"
              geometry_line: "LINESTRING(1 2,3 4)"
              geometry_poly: "POLYGON((1 2,3 4,5 6,1 2))"
              geometry_multipoint: "MULTIPOINT(1 2)"
              geometry_multiline: "MULTILINESTRING((1 2,3 4))"
              geometry_multipoly: "MULTIPOLYGON(((1 2,3 4,5 6,1 2)))"
              geometry_collection: "GEOMETRYCOLLECTION(POINT(1 2))"
            }
          ) {
            geometry
            geometry_point
            geometry_line
            geometry_poly
            geometry_multipoint
            geometry_multiline
            geometry_multipoly
            geometry_collection
          }
        }"#),
            @r###"{"data":{"createOneModel":{"geometry":"POINT(1 2)","geometry_point":"POINT(1 2)","geometry_line":"LINESTRING(1 2,3 4)","geometry_poly":"POLYGON((1 2,3 4,5 6,1 2))","geometry_multipoint":"MULTIPOINT(1 2)","geometry_multiline":"MULTILINESTRING((1 2,3 4))","geometry_multipoly":"MULTIPOLYGON(((1 2,3 4,5 6,1 2)))","geometry_collection":"GEOMETRYCOLLECTION(POINT(1 2))"}}}"###
        );

        Ok(())
    }

    fn schema_ewkt_geometry_srid() -> String {
        let schema = indoc! {
            r#"model Model {
            @@schema("test")
            #id(id, String, @id, @default(cuid()))
            geometry             Geometry @test.Geometry(Geometry, 3857)
            geometry_point       Geometry @test.Geometry(Point, 3857)
            geometry_line        Geometry @test.Geometry(LineString, 3857)
            geometry_poly        Geometry @test.Geometry(Polygon, 3857)
            geometry_multipoint  Geometry @test.Geometry(MultiPoint, 3857)
            geometry_multiline   Geometry @test.Geometry(MultiLineString, 3857)
            geometry_multipoly   Geometry @test.Geometry(MultiPolygon, 3857)
            geometry_collection  Geometry @test.Geometry(GeometryCollection, 3857)
          }"#
        };

        schema.to_owned()
    }

    // "PostGIS common geometry types with srid" should "work"
    #[connector_test(
        only(Postgres("15-postgis"), CockroachDb),
        schema(schema_ewkt_geometry_srid),
        db_schemas("public", "test")
    )]
    async fn native_geometry_srid(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
          createOneModel(
            data: {
              geometry: "SRID=3857;POINT(1 2)"
              geometry_point: "SRID=3857;POINT(1 2)"
              geometry_line: "SRID=3857;LINESTRING(1 2,3 4)"
              geometry_poly: "SRID=3857;POLYGON((1 2,3 4,5 6,1 2))"
              geometry_multipoint: "SRID=3857;MULTIPOINT(1 2)"
              geometry_multiline: "SRID=3857;MULTILINESTRING((1 2,3 4))"
              geometry_multipoly: "SRID=3857;MULTIPOLYGON(((1 2,3 4,5 6,1 2)))"
              geometry_collection: "SRID=3857;GEOMETRYCOLLECTION(POINT(1 2))"
            }
          ) {
            geometry
            geometry_point
            geometry_line
            geometry_poly
            geometry_multipoint
            geometry_multiline
            geometry_multipoly
            geometry_collection
          }
        }"#),
            @r###"{"data":{"createOneModel":{"geometry":"SRID=3857;POINT(1 2)","geometry_point":"SRID=3857;POINT(1 2)","geometry_line":"SRID=3857;LINESTRING(1 2,3 4)","geometry_poly":"SRID=3857;POLYGON((1 2,3 4,5 6,1 2))","geometry_multipoint":"SRID=3857;MULTIPOINT(1 2)","geometry_multiline":"SRID=3857;MULTILINESTRING((1 2,3 4))","geometry_multipoly":"SRID=3857;MULTIPOLYGON(((1 2,3 4,5 6,1 2)))","geometry_collection":"SRID=3857;GEOMETRYCOLLECTION(POINT(1 2))"}}}"###
        );

        Ok(())
    }

    fn schema_ewkt_geography() -> String {
        let schema = indoc! {
            r#"model Model {
            @@schema("test")
            #id(id, String, @id, @default(cuid()))
            geography            Geometry @test.Geography(Geometry)
            geography_point      Geometry @test.Geography(Point)
            geography_line       Geometry @test.Geography(LineString)
            geography_poly       Geometry @test.Geography(Polygon)
            geography_multipoint Geometry @test.Geography(MultiPoint)
            geography_multiline  Geometry @test.Geography(MultiLineString)
            geography_multipoly  Geometry @test.Geography(MultiPolygon)
            geography_collection Geometry @test.Geography(GeometryCollection)
          }"#
        };

        schema.to_owned()
    }

    // "PostGIS common geography types" should "work"
    #[connector_test(
        only(Postgres("15-postgis"), CockroachDb),
        schema(schema_ewkt_geography),
        db_schemas("public", "test")
    )]
    async fn native_ewkt_geography(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
          createOneModel(
            data: {
              geography: "SRID=4326;POINT(1 2)"
              geography_point: "SRID=4326;POINT(1 2)"
              geography_line: "SRID=4326;LINESTRING(1 2,3 4)"
              geography_poly: "SRID=4326;POLYGON((1 2,3 4,5 6,1 2))"
              geography_multipoint: "SRID=4326;MULTIPOINT(1 2)"
              geography_multiline: "SRID=4326;MULTILINESTRING((1 2,3 4))"
              geography_multipoly: "SRID=4326;MULTIPOLYGON(((1 2,3 4,5 6,1 2)))"
              geography_collection: "SRID=4326;GEOMETRYCOLLECTION(POINT(1 2))"
            }
          ) {
            geography
            geography_point
            geography_line
            geography_poly
            geography_multipoint
            geography_multiline
            geography_multipoly
            geography_collection
          }
        }"#),
            @r###"{"data":{"createOneModel":{"geography":"SRID=4326;POINT(1 2)","geography_point":"SRID=4326;POINT(1 2)","geography_line":"SRID=4326;LINESTRING(1 2,3 4)","geography_poly":"SRID=4326;POLYGON((1 2,3 4,5 6,1 2))","geography_multipoint":"SRID=4326;MULTIPOINT(1 2)","geography_multiline":"SRID=4326;MULTILINESTRING((1 2,3 4))","geography_multipoly":"SRID=4326;MULTIPOLYGON(((1 2,3 4,5 6,1 2)))","geography_collection":"SRID=4326;GEOMETRYCOLLECTION(POINT(1 2))"}}}"###
        );

        Ok(())
    }

    fn schema_ewkt_geography_srid() -> String {
        let schema = indoc! {
            r#"model Model {
            @@schema("test")
            #id(id, String, @id, @default(cuid()))
            geography            Geometry @test.Geography(Geometry, 9000)
            geography_point      Geometry @test.Geography(Point, 9000)
            geography_line       Geometry @test.Geography(LineString, 9000)
            geography_poly       Geometry @test.Geography(Polygon, 9000)
            geography_multipoint Geometry @test.Geography(MultiPoint, 9000)
            geography_multiline  Geometry @test.Geography(MultiLineString, 9000)
            geography_multipoly  Geometry @test.Geography(MultiPolygon, 9000)
            geography_collection Geometry @test.Geography(GeometryCollection, 9000)
          }"#
        };

        schema.to_owned()
    }

    // "PostGIS common geography types with srid" should "work"
    #[connector_test(
        only(Postgres("15-postgis"), CockroachDb),
        schema(schema_ewkt_geography_srid),
        db_schemas("public", "test")
    )]
    async fn native_ewkt_geography_srid(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
          createOneModel(
            data: {
              geography: "SRID=9000;POINT(1 2)"
              geography_point: "SRID=9000;POINT(1 2)"
              geography_line: "SRID=9000;LINESTRING(1 2,3 4)"
              geography_poly: "SRID=9000;POLYGON((1 2,3 4,5 6,1 2))"
              geography_multipoint: "SRID=9000;MULTIPOINT(1 2)"
              geography_multiline: "SRID=9000;MULTILINESTRING((1 2,3 4))"
              geography_multipoly: "SRID=9000;MULTIPOLYGON(((1 2,3 4,5 6,1 2)))"
              geography_collection: "SRID=9000;GEOMETRYCOLLECTION(POINT(1 2))"
            }
          ) {
            geography
            geography_point
            geography_line
            geography_poly
            geography_multipoint
            geography_multiline
            geography_multipoly
            geography_collection
          }
        }"#),
            @r###"{"data":{"createOneModel":{"geography":"SRID=9000;POINT(1 2)","geography_point":"SRID=9000;POINT(1 2)","geography_line":"SRID=9000;LINESTRING(1 2,3 4)","geography_poly":"SRID=9000;POLYGON((1 2,3 4,5 6,1 2))","geography_multipoint":"SRID=9000;MULTIPOINT(1 2)","geography_multiline":"SRID=9000;MULTILINESTRING((1 2,3 4))","geography_multipoly":"SRID=9000;MULTIPOLYGON(((1 2,3 4,5 6,1 2)))","geography_collection":"SRID=9000;GEOMETRYCOLLECTION(POINT(1 2))"}}}"###
        );

        Ok(())
    }

    fn schema_extra_geometry() -> String {
        let schema = indoc! {
            r#"model Model {
            @@schema("test")
            #id(id, String, @id, @default(cuid()))
            geometry_triangle       Geometry @test.Geometry(Triangle)
            geometry_circularstring Geometry @test.Geometry(CircularString)
            geometry_compoundcurve  Geometry @test.Geometry(CompoundCurve)
            geometry_curvepolygon   Geometry @test.Geometry(CurvePolygon)
            geometry_multicurve     Geometry @test.Geometry(MultiCurve)
            geometry_multisurface   Geometry @test.Geometry(MultiSurface)
            geometry_polyhedral     Geometry @test.Geometry(PolyhedralSurfaceZ)
            geometry_tin            Geometry @test.Geometry(Tin)
          }"#
        };

        schema.to_owned()
    }

    // "PostGIS extra geometry types" should "work"
    #[connector_test(
        only(Postgres("15-postgis")),
        schema(schema_extra_geometry),
        db_schemas("public", "test")
    )]
    async fn native_extra_geometry(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
          createOneModel(
            data: {
              geometry_triangle: "TRIANGLE((0 0,1 1,2 0,0 0))"
              geometry_circularstring: "CIRCULARSTRING(0 0,4 0,4 4,0 4,0 0)"
              geometry_compoundcurve: "COMPOUNDCURVE(CIRCULARSTRING(0 0,1 1,1 0),(1 0,0 1))"
              geometry_curvepolygon: "CURVEPOLYGON(CIRCULARSTRING(0 0,4 0,4 4,0 4,0 0),(1 1,3 3,3 1,1 1))"
              geometry_multicurve: "MULTICURVE((0 0,5 5),CIRCULARSTRING(4 0,4 4,8 4))"
              geometry_multisurface: "MULTISURFACE(CURVEPOLYGON(CIRCULARSTRING(0 0,4 0,4 4,0 4,0 0),(1 1,3 3,3 1,1 1)),((10 10,14 12,11 10,10 10),(11 11,11.5 11,11 11.5,11 11)))"
              geometry_polyhedral:"POLYHEDRALSURFACE(((0 0 0,1 0 0,0 1 0,0 0 1,0 0 0)))"
              geometry_tin: "TIN(((80 130,50 160,80 70,80 130)),((50 160,10 190,10 70,50 160)),((80 70,50 160,10 70,80 70)),((120 160,120 190,50 160,120 160)),((120 190,10 190,50 160,120 190)))"
            }
          ) {
            geometry_triangle
            geometry_circularstring
            geometry_compoundcurve
            geometry_curvepolygon
            geometry_multicurve
            geometry_multisurface
            geometry_polyhedral
            geometry_tin
          }
        }"#),
            @r###"{"data":{"createOneModel":{"geometry_triangle":"TRIANGLE((0 0,1 1,2 0,0 0))","geometry_circularstring":"CIRCULARSTRING(0 0,4 0,4 4,0 4,0 0)","geometry_compoundcurve":"COMPOUNDCURVE(CIRCULARSTRING(0 0,1 1,1 0),(1 0,0 1))","geometry_curvepolygon":"CURVEPOLYGON(CIRCULARSTRING(0 0,4 0,4 4,0 4,0 0),(1 1,3 3,3 1,1 1))","geometry_multicurve":"MULTICURVE((0 0,5 5),CIRCULARSTRING(4 0,4 4,8 4))","geometry_multisurface":"MULTISURFACE(CURVEPOLYGON(CIRCULARSTRING(0 0,4 0,4 4,0 4,0 0),(1 1,3 3,3 1,1 1)),((10 10,14 12,11 10,10 10),(11 11,11.5 11,11 11.5,11 11)))","geometry_polyhedral":"POLYHEDRALSURFACE(((0 0 0,1 0 0,0 1 0,0 0 1,0 0 0)))","geometry_tin":"TIN(((80 130,50 160,80 70,80 130)),((50 160,10 190,10 70,50 160)),((80 70,50 160,10 70,80 70)),((120 160,120 190,50 160,120 160)),((120 190,10 190,50 160,120 190)))"}}}"###
        );

        Ok(())
    }

    fn schema_geojson_geometry() -> String {
        let schema = indoc! {
            r#"model Model {
            @@schema("test")
            #id(id, String, @id, @default(cuid()))
            geometry             GeoJson @test.Geometry(Geometry, 4326)
            geometry_point       GeoJson @test.Geometry(Point, 4326)
            geometry_line        GeoJson @test.Geometry(LineString, 4326)
            geometry_poly        GeoJson @test.Geometry(Polygon, 4326)
            geometry_multipoint  GeoJson @test.Geometry(MultiPoint, 4326)
            geometry_multiline   GeoJson @test.Geometry(MultiLineString, 4326)
            geometry_multipoly   GeoJson @test.Geometry(MultiPolygon, 4326)
            geometry_collection  GeoJson @test.Geometry(GeometryCollection, 4326)
          }"#
        };

        schema.to_owned()
    }

    // "PostGIS common geometry types" should "work" with GeoJSON
    #[connector_test(
        only(Postgres("15-postgis"), CockroachDb),
        schema(schema_geojson_geometry),
        db_schemas("public", "test")
    )]
    async fn native_geojson_geometry(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
          createOneModel(
            data: {
              geometry: "{\"type\":\"Point\",\"coordinates\":[1,2]}"
              geometry_point: "{\"type\":\"Point\",\"coordinates\":[1,2]}"
              geometry_line: "{\"type\":\"LineString\",\"coordinates\":[[1,2],[3,4]]}"
              geometry_poly: "{\"type\":\"Polygon\",\"coordinates\":[[[1,2],[3,4],[5,6],[1,2]]]}"
              geometry_multipoint: "{\"type\":\"MultiPoint\",\"coordinates\":[[1,2]]}"
              geometry_multiline: "{\"type\":\"MultiLineString\",\"coordinates\":[[[1,2],[3,4]]]}"
              geometry_multipoly: "{\"type\":\"MultiPolygon\",\"coordinates\":[[[[1,2],[3,4],[5,6],[1,2]]]]}"
              geometry_collection: "{\"type\":\"GeometryCollection\",\"geometries\":[{\"type\":\"Point\",\"coordinates\":[1,2]}]}"
            }
          ) {
            geometry
            geometry_point
            geometry_line
            geometry_poly
            geometry_multipoint
            geometry_multiline
            geometry_multipoly
            geometry_collection
          }
        }"#),
        @r###"{"data":{"createOneModel":{"geometry":"{\"type\": \"Point\", \"coordinates\": [1,2]}","geometry_point":"{\"type\": \"Point\", \"coordinates\": [1,2]}","geometry_line":"{\"type\": \"LineString\", \"coordinates\": [[1,2],[3,4]]}","geometry_poly":"{\"type\": \"Polygon\", \"coordinates\": [[[1,2],[3,4],[5,6],[1,2]]]}","geometry_multipoint":"{\"type\": \"MultiPoint\", \"coordinates\": [[1,2]]}","geometry_multiline":"{\"type\": \"MultiLineString\", \"coordinates\": [[[1,2],[3,4]]]}","geometry_multipoly":"{\"type\": \"MultiPolygon\", \"coordinates\": [[[[1,2],[3,4],[5,6],[1,2]]]]}","geometry_collection":"{\"type\": \"GeometryCollection\", \"geometries\": [{\"type\": \"Point\", \"coordinates\": [1,2]}]}"}}}"###
        );

        Ok(())
    }

    fn schema_geojson_geography() -> String {
        let schema = indoc! {
            r#"model Model {
            @@schema("test")
            #id(id, String, @id, @default(cuid()))
            geography             GeoJson @test.Geography(Geometry, 4326)
            geography_point       GeoJson @test.Geography(Point, 4326)
            geography_line        GeoJson @test.Geography(LineString, 4326)
            geography_poly        GeoJson @test.Geography(Polygon, 4326)
            geography_multipoint  GeoJson @test.Geography(MultiPoint, 4326)
            geography_multiline   GeoJson @test.Geography(MultiLineString, 4326)
            geography_multipoly   GeoJson @test.Geography(MultiPolygon, 4326)
            geography_collection  GeoJson @test.Geography(GeometryCollection, 4326)
          }"#
        };

        schema.to_owned()
    }

    // "PostGIS common geometry types" should "work" with GeoJSON
    #[connector_test(
        only(Postgres("15-postgis"), CockroachDb),
        schema(schema_geojson_geography),
        db_schemas("public", "test")
    )]
    async fn native_geojson_geography(runner: Runner) -> TestResult<()> {
        insta::assert_snapshot!(
          run_query!(&runner, r#"mutation {
          createOneModel(
            data: {
              geography: "{\"type\":\"Point\",\"coordinates\":[1,2]}"
              geography_point: "{\"type\":\"Point\",\"coordinates\":[1,2]}"
              geography_line: "{\"type\":\"LineString\",\"coordinates\":[[1,2],[3,4]]}"
              geography_poly: "{\"type\":\"Polygon\",\"coordinates\":[[[1,2],[3,4],[5,6],[1,2]]]}"
              geography_multipoint: "{\"type\":\"MultiPoint\",\"coordinates\":[[1,2]]}"
              geography_multiline: "{\"type\":\"MultiLineString\",\"coordinates\":[[[1,2],[3,4]]]}"
              geography_multipoly: "{\"type\":\"MultiPolygon\",\"coordinates\":[[[[1,2],[3,4],[5,6],[1,2]]]]}"
              geography_collection: "{\"type\":\"GeometryCollection\",\"geometries\":[{\"type\":\"Point\",\"coordinates\":[1,2]}]}"
            }
          ) {
            geography
            geography_point
            geography_line
            geography_poly
            geography_multipoint
            geography_multiline
            geography_multipoly
            geography_collection
          }
        }"#),
        @r###"{"data":{"createOneModel":{"geography":"{\"type\": \"Point\", \"coordinates\": [1,2]}","geography_point":"{\"type\": \"Point\", \"coordinates\": [1,2]}","geography_line":"{\"type\": \"LineString\", \"coordinates\": [[1,2],[3,4]]}","geography_poly":"{\"type\": \"Polygon\", \"coordinates\": [[[1,2],[3,4],[5,6],[1,2]]]}","geography_multipoint":"{\"type\": \"MultiPoint\", \"coordinates\": [[1,2]]}","geography_multiline":"{\"type\": \"MultiLineString\", \"coordinates\": [[[1,2],[3,4]]]}","geography_multipoly":"{\"type\": \"MultiPolygon\", \"coordinates\": [[[[1,2],[3,4],[5,6],[1,2]]]]}","geography_collection":"{\"type\": \"GeometryCollection\", \"geometries\": [{\"type\": \"Point\", \"coordinates\": [1,2]}]}"}}}"###
        );

        Ok(())
    }
}
