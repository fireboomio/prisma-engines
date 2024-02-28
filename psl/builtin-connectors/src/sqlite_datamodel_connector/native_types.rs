use crate::geometry::GeometryParams;

crate::native_type_definition! {
    /// The SQLite native type enum.
    SQLiteType;
    Geometry(Option<GeometryParams>) -> Geometry | GeoJson,
}
