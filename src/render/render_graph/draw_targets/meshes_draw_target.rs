use crate::{
    asset::{Handle, Mesh},
    legion::prelude::*,
    render::{
        render_graph::{PipelineDescriptor, RenderPass, Renderable, ResourceInfo},
        Instanced,
    },
};

pub fn meshes_draw_target(
    world: &World,
    render_pass: &mut dyn RenderPass,
    _pipeline_handle: Handle<PipelineDescriptor>,
) {
    let mut current_mesh_handle = None;
    let mut current_mesh_index_len = 0;
    let mesh_query =
        <(Read<Handle<Mesh>>, Read<Renderable>)>::query().filter(!component::<Instanced>());

    for (entity, (mesh, renderable)) in mesh_query.iter_entities(world) {
        if !renderable.is_visible {
            continue;
        }

        let renderer = render_pass.get_renderer();
        let render_resources = renderer.get_render_resources();
        if current_mesh_handle != Some(*mesh) {
            if let Some(vertex_buffer_resource) = render_resources.get_mesh_vertices_resource(*mesh)
            {
                let index_buffer_resource =
                    render_resources.get_mesh_indices_resource(*mesh).unwrap();
                match renderer.get_resource_info(index_buffer_resource).unwrap() {
                    ResourceInfo::Buffer { size, .. } => current_mesh_index_len = (size / 2) as u32,
                    _ => panic!("expected a buffer type"),
                }
                render_pass.set_index_buffer(index_buffer_resource, 0);
                render_pass.set_vertex_buffer(0, vertex_buffer_resource, 0);
            }
            // TODO: Verify buffer format matches render pass
            current_mesh_handle = Some(*mesh);
        }

        // TODO: validate bind group properties against shader uniform properties at least once
        render_pass.setup_bind_groups(Some(&entity));
        render_pass.draw_indexed(0..current_mesh_index_len, 0, 0..1);
    }
}