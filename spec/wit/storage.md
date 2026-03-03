# WIT Storage

The WIT metadata storage layer persists WebAssembly Interface Types data.

r[wit.world.insert]
WIT worlds MUST be insertable and queryable.

r[wit.world.imports-exports]
WIT world imports and exports MUST be storable.

r[wit.world.idempotent]
Import and export operations MUST be idempotent.

r[wit.interface.dependencies]
WIT interface dependencies MUST be storable.

r[wit.component.insert]
WASM components and their targets MUST be storable.

r[wit.component.wit-only]
WIT-only packages MUST NOT create component rows.

## Foreign Key Resolution

r[wit.resolve.import]
Import resolution MUST populate `resolved_interface_id` when the dependency exists.

r[wit.resolve.import-missing]
Import resolution MUST leave the field NULL when the dependency is missing.

r[wit.resolve.dependency]
Dependency interface IDs MUST be resolvable.

r[wit.resolve.export]
Export interface IDs MUST be resolvable.

r[wit.resolve.component-target]
Component targets MUST be resolvable across packages.
