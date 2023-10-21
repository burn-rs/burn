@group(0)
@binding(0)
var<storage, read_write> input: array<{{ elem }}>;

@group(0)
@binding(1)
var<storage, read> min_value: f32;

@group(0)
@binding(2)
var<storage, read> max_value: f32;

@compute
@workgroup_size({{ workgroup_size_x }}, {{ workgroup_size_y }}, 1)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>, 
    @builtin(num_workgroups) num_workgroups: vec3<u32>,
) {
    let id = global_id.y * (num_workgroups.x * {{ workgroup_size_x }}u) + global_id.x;
    input[id] = clamp(input[id], min_value, max_value);
}
