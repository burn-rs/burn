use super::io::*;
use super::ir::*;
use cubecl::prelude::*;
use half::{bf16, f16};

const DYN_ELEM_ID: u8 = u8::MAX;

#[cube]
/// Fuse element-wise operations at the given write position.
///
/// You can start by writing some elements using `write_values` and `write_args`.
pub fn fuse_on_write<E: CubePrimitive>(
    inputs: &GlobalArgs,
    outputs: &mut GlobalArgs,
    write_pos: u32,
    write_values: Registry<Arg, Line<E>>,
    #[comptime] write_args: Sequence<Arg>,
    #[comptime] config: &ElemwiseConfig,
) {
    let mut locals = LocalArgs {
        l_f32: Registry::<u32, Line<f32>>::new(),
        l_f16: Registry::<u32, Line<f16>>::new(),
        l_bf16: Registry::<u32, Line<bf16>>::new(),
        l_i64: Registry::<u32, Line<i64>>::new(),
        l_i32: Registry::<u32, Line<i32>>::new(),
        l_i16: Registry::<u32, Line<i16>>::new(),
        l_i8: Registry::<u32, Line<i8>>::new(),
        l_u64: Registry::<u32, Line<u64>>::new(),
        l_u32: Registry::<u32, Line<u32>>::new(),
        l_u16: Registry::<u32, Line<u16>>::new(),
        l_u8: Registry::<u32, Line<u8>>::new(),
        l_bool: Registry::<u32, Line<bool>>::new(),
    };

    // Write the values given as arguments.
    #[unroll]
    for i in 0..write_args.len() {
        let arg = comptime![write_args.index(i).clone()];
        let val = write_values.find(comptime![arg.clone()]);

        write::<E>(inputs, outputs, &mut locals, write_pos, val, arg, config);
    }

    #[unroll]
    for index in 0..config.ops.len() {
        let op = comptime! { config.ops.index(index).clone() };
        set_polyfill::<NumericExpand<DYN_ELEM_ID>>(comptime![op.cmp_elem()]);

        match op {
            ElemwiseOp::Add(op) => add::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Div(op) => div::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Sub(op) => sub::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Mul(op) => mul::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Powf(op) => powf::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Erf(op) => erf::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Abs(op) => abs::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Log(op) => log::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Log1p(op) => log1p::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Recip(op) => recip::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Assign(op) => assign::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Exp(op) => exp::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Cos(op) => cos::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Sin(op) => sin::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Tanh(op) => tanh::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Equal(op) => equal::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Greater(op) => greater::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::GreaterEqual(op) => greater_equal::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::Lower(op) => lower::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::LowerEqual(op) => lower_equal::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                op,
                config,
            ),
            ElemwiseOp::ConditionalAssign {
                cond,
                lhs,
                rhs,
                out,
            } => conditional_assign::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                cond,
                lhs,
                rhs,
                out,
                config,
            ),
            ElemwiseOp::Gather {
                input,
                indices,
                output,
                dim,
            } => gather::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                dim,
                input,
                indices,
                output,
                config,
            ),
            ElemwiseOp::Select {
                input,
                indices,
                output,
                dim,
            } => select_indices::<NumericExpand<DYN_ELEM_ID>>(
                inputs,
                outputs,
                &mut locals,
                write_pos,
                dim,
                input,
                indices,
                output,
                config,
            ),
        }
    }
}

macro_rules! binary_op {
    ($ident:ident, $op:tt) => {
        #[cube]
        fn $ident<C: Numeric>(
            inputs: &GlobalArgs,
            outputs: &mut GlobalArgs,
            locals: &mut LocalArgs,
            write_pos: u32,
            #[comptime] op: BinaryElemwiseArgs,
            #[comptime] config: &ElemwiseConfig,
        ) {
            let lhs = read::<C>(inputs, outputs, &locals, write_pos, op.lhs, config);
            let rhs = read::<C>(inputs, outputs, &locals, write_pos, op.rhs, config);
            let result = lhs $op rhs;

            write::<C>(inputs, outputs, locals, write_pos, result, op.out, config);
        }
    };
}

macro_rules! binary_func {
    ($ident:ident, $func:expr, $c:tt) => {
        #[cube]
        fn $ident<C: $c>(
            inputs: &GlobalArgs,
            outputs: &mut GlobalArgs,
            locals: &mut LocalArgs,
            write_pos: u32,
            #[comptime] op: BinaryElemwiseArgs,
            #[comptime] config: &ElemwiseConfig,
        ) {
            let lhs = read::<C>(inputs, outputs, &locals, write_pos, op.lhs, config);
            let rhs = read::<C>(inputs, outputs, &locals, write_pos, op.rhs, config);
            let result = $func(lhs, rhs);

            write::<C>(inputs, outputs, locals, write_pos, result, op.out, config);
        }
    };
}

macro_rules! comparison_op {
    ($ident:ident, $op:tt) => {
        #[cube]
        fn $ident<C: CubePrimitive + core::cmp::PartialOrd>(
            inputs: &GlobalArgs,
            outputs: &mut GlobalArgs,
            locals: &mut LocalArgs,
            write_pos: u32,
            #[comptime] op: BinaryElemwiseArgs,
            #[comptime] config: &ElemwiseConfig,
        ) {
            let lhs = read::<C>(inputs, outputs, &locals, write_pos, op.lhs, config);
            let rhs = read::<C>(inputs, outputs, &locals, write_pos, op.rhs, config);
            let result = Line::new(lhs $op rhs);

            write::<bool>(inputs, outputs, locals, write_pos, result, op.out, config);
        }
    };
}

macro_rules! unary_func {
    ($ident:ident, $func:expr, $c:tt) => {
        #[cube]
        fn $ident<C: $c>(
            inputs: &GlobalArgs,
            outputs: &mut GlobalArgs,
            locals: &mut LocalArgs,
            write_pos: u32,
            #[comptime] op: UnaryElemwiseArgs,
            #[comptime] config: &ElemwiseConfig,
        ) {
            let input = read::<C>(inputs, outputs, &locals, write_pos, op.input, config);
            let result = $func(input);

            write::<C>(inputs, outputs, locals, write_pos, result, op.out, config);
        }
    };
}

#[cube]
fn assign<C: CubePrimitive>(
    inputs: &GlobalArgs,
    outputs: &mut GlobalArgs,
    locals: &mut LocalArgs,
    write_pos: u32,
    #[comptime] op: UnaryElemwiseArgs,
    #[comptime] config: &ElemwiseConfig,
) {
    let input = read::<C>(inputs, outputs, locals, write_pos, op.input, config);

    write::<C>(inputs, outputs, locals, write_pos, input, op.out, config);
}

#[cube]
fn gather<C: Numeric>(
    inputs: &GlobalArgs,
    outputs: &mut GlobalArgs,
    locals: &mut LocalArgs,
    write_pos: u32,
    #[comptime] dim: u32,
    #[comptime] input: Arg,
    #[comptime] indices: Arg,
    #[comptime] output: Arg,
    #[comptime] config: &ElemwiseConfig,
) {
    let mut index = read::<u32>(inputs, outputs, locals, write_pos, indices, config);
    let (pos, precision) = comptime! {
        match input {
            Arg::Input(pos, precision, _) => (pos, precision),
            _ => panic!("Input tensor isn't an input"),
        }
    };
    let line_size = match config.ref_layout {
        Arg::Input(pos, precision, _) => global_line_size(inputs, pos, precision),
        Arg::Output(pos, precision, _) => global_line_size(outputs, pos, precision),
        _ => unreachable!(),
    };
    let stride = global_stride(inputs, dim, pos, precision);

    index *= Line::new(stride);

    if comptime![dim > 0] {
        let index_before = global_offset(
            inputs,
            outputs,
            write_pos,
            comment!(input.clone()),
            comptime![Some((0u32, dim))],
            config,
        );
        index += Line::new(index_before);
    }

    if comptime![dim + 1 < config.rank] {
        let index_after = global_offset(
            inputs,
            outputs,
            write_pos,
            input,
            comptime![Some((dim + 1, config.rank))],
            config,
        );
        index += Line::new(index_after);
    }

    let mut result = Line::empty(line_size);

    #[unroll]
    for i in 0..line_size {
        let index = index[i];

        let input = read_input::<C>(
            inputs,
            outputs,
            pos,
            index,
            LayoutInfo::IsRef,
            precision,
            config,
            None,
        );
        result[i] = input[0];
    }

    write::<C>(inputs, outputs, locals, write_pos, result, output, config);
}

#[cube]
fn select_indices<C: Numeric>(
    inputs: &GlobalArgs,
    outputs: &mut GlobalArgs,
    locals: &mut LocalArgs,
    write_pos: u32,
    #[comptime] dim: u32,
    #[comptime] input: Arg,
    #[comptime] indices: Arg,
    #[comptime] output: Arg,
    #[comptime] config: &ElemwiseConfig,
) {
    let (line_size_ref, stride_dim_ref, shape_dim_ref) = match config.ref_layout {
        Arg::Input(pos, precision, _) => (
            global_line_size(inputs, pos, precision),
            global_stride(inputs, dim, pos, precision),
            global_shape(inputs, dim, pos, precision),
        ),
        Arg::Output(pos, precision, _) => (
            global_line_size(outputs, pos, precision),
            global_stride(outputs, dim, pos, precision),
            global_shape(outputs, dim, pos, precision),
        ),
        _ => unreachable!(),
    };

    let (pos_input, precision_input) = comptime! {
        match input {
            Arg::Input(pos, precision, _) => (pos, precision),
            _ => panic!("Input tensor isn't an input"),
        }
    };
    let (pos_indices, precision_indices) = match indices {
        Arg::Input(pos, precision, ..) => (pos, precision),
        _ => panic!("Indices tensor isn't an input"),
    };

    let stride_input_dim = global_stride(inputs, dim, pos_input, precision_input);

    let mut index = 0u32;
    let mut result = Line::empty(line_size_ref);

    if comptime![dim != config.rank - 1] {
        // In this scenario the select is actually broadcasted along the axis we're working on.
        //
        // Therefore the same indices are used to fetch multiple entries in the input tensor.

        let write_pos_input = write_pos * line_size_ref;
        let stride_input_line = global_stride(
            inputs,
            comptime![config.rank - 1],
            pos_input,
            precision_input,
        );

        if comptime![dim > 0] {
            let index_before = global_offset(
                inputs,
                outputs,
                write_pos_input,
                comment!(input.clone()),
                comptime![Some((0u32, dim))],
                config,
            );
            index += index_before;
        }

        if comptime![dim + 1 < config.rank] {
            let index_after = global_offset(
                inputs,
                outputs,
                write_pos_input,
                comment!(input.clone()),
                comptime![Some((dim + 1, config.rank))],
                config,
            );
            index += index_after;
        }

        let coordinate_dim = write_pos_input / stride_dim_ref % shape_dim_ref;
        let offset_dim = read_input::<u32>(
            inputs,
            outputs,
            pos_indices,
            coordinate_dim,
            LayoutInfo::IsRef,
            precision_indices,
            config,
            None,
        );

        index *= line_size_ref;
        index += offset_dim[0] * stride_input_dim;

        #[unroll]
        for i in 0..line_size_ref {
            let input = read_input::<C>(
                inputs,
                outputs,
                pos_input,
                index + i * stride_input_line,
                LayoutInfo::IsRef,
                precision_input,
                config,
                None,
            );
            result[i] = input[0];
        }
    } else {
        // In this scenario the select is actually performed on the last dimension we're working on.
        //
        // Therefore we need to fetch multiple indices that correspond to different entries in the
        // input tensor.

        if comptime![dim > 0] {
            let index_before = global_offset(
                inputs,
                outputs,
                write_pos,
                comment!(input.clone()),
                comptime![Some((0u32, dim))],
                config,
            );
            index += index_before;
        }

        if comptime![dim + 1 < config.rank] {
            let index_after = global_offset(
                inputs,
                outputs,
                write_pos,
                input,
                comptime![Some((dim + 1, config.rank))],
                config,
            );
            index += index_after;
        }

        let write_pos_indices = write_pos * line_size_ref;

        #[unroll]
        for i in 0..line_size_ref {
            let coordinate_dim = (write_pos_indices + i) / stride_dim_ref % shape_dim_ref;
            let offset_dim = read_input::<u32>(
                inputs,
                outputs,
                pos_indices,
                coordinate_dim,
                LayoutInfo::IsRef,
                precision_indices,
                config,
                None,
            );

            let input = read_input::<C>(
                inputs,
                outputs,
                pos_input,
                index + (offset_dim[0] * stride_input_dim),
                LayoutInfo::IsRef,
                precision_input,
                config,
                None,
            );
            result[i] = input[0];
        }
    }

    write::<C>(inputs, outputs, locals, write_pos, result, output, config);
}

#[cube]
fn conditional_assign<C: CubePrimitive>(
    inputs: &GlobalArgs,
    outputs: &mut GlobalArgs,
    locals: &mut LocalArgs,
    write_pos: u32,
    #[comptime] cond: Arg,
    #[comptime] lhs: Arg,
    #[comptime] rhs: Arg,
    #[comptime] out: Arg,
    #[comptime] config: &ElemwiseConfig,
) {
    let cond = read::<bool>(inputs, outputs, locals, write_pos, cond, config);
    let lhs = read::<C>(inputs, outputs, locals, write_pos, lhs, config);
    let rhs = read::<C>(inputs, outputs, locals, write_pos, rhs, config);
    let result = select_many(cond, lhs, rhs);

    write::<C>(inputs, outputs, locals, write_pos, result, out, config);
}

binary_op!(add, +);
binary_op!(mul, *);
binary_op!(div, /);
binary_op!(sub, -);

comparison_op!(equal, ==);
comparison_op!(greater, >);
comparison_op!(greater_equal, >=);
comparison_op!(lower, <);
comparison_op!(lower_equal, <=);

binary_func!(powf, Line::<C>::powf, Float);

unary_func!(exp, Line::<C>::exp, Float);
unary_func!(log, Line::<C>::log, Float);
unary_func!(log1p, Line::<C>::log1p, Float);
unary_func!(cos, Line::<C>::cos, Float);
unary_func!(sin, Line::<C>::sin, Float);
unary_func!(tanh, Line::<C>::tanh, Float);
unary_func!(erf, Line::<C>::erf, Float);
unary_func!(recip, Line::<C>::recip, Float);
unary_func!(abs, Line::<C>::abs, Numeric);
