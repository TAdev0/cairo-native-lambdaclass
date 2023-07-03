//! # Branch alignment libfunc
//!
//! Natively compiled code doesn't need branch alignment because it has no notion of segments.
//! Because of this, this libfunc is a no-op.

use super::{LibfuncBuilder, LibfuncHelper};
use crate::{metadata::MetadataStorage, types::TypeBuilder};
use cairo_lang_sierra::{
    extensions::{lib_func::SignatureOnlyConcreteLibfunc, GenericLibfunc, GenericType},
    program_registry::ProgramRegistry,
};
use melior::{
    ir::{Block, Location},
    Context,
};

/// Generate MLIR operations for the `branch_align` libfunc.
pub fn build<'ctx, 'this, TType, TLibfunc>(
    _context: &'ctx Context,
    _registry: &ProgramRegistry<TType, TLibfunc>,
    entry: &'this Block<'ctx>,
    location: Location<'ctx>,
    helper: &LibfuncHelper<'ctx, 'this>,
    _metadata: &mut MetadataStorage,
    _info: &SignatureOnlyConcreteLibfunc,
) -> Result<(), std::convert::Infallible>
where
    TType: GenericType,
    TLibfunc: GenericLibfunc,
    <TType as GenericType>::Concrete: TypeBuilder,
    <TLibfunc as GenericLibfunc>::Concrete: LibfuncBuilder,
{
    entry.append_operation(helper.br(0, &[], location));

    Ok(())
}