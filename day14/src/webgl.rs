use derive_more::Add;
use js_sys::Math;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::WebGlUniformLocation;

#[derive(Add, Clone, Debug)]
pub struct IntVec2 {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug)]
pub struct Rectangle {
    pub pos: IntVec2,
    pub width: i32,
    pub height: i32,
    pub color: [f32; 4],
}

pub struct Webgl {
    context: WebGl2RenderingContext,
    position_attribute_location: i32,
    resolution_uniform_location: WebGlUniformLocation,
    color_location: WebGlUniformLocation,
}
impl Webgl {
    pub fn new(canvas: HtmlCanvasElement, width: i32, height: i32) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()?;

        let vert_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("shaders/rectangle.vs"),
        )?;
        let frag_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("shaders/rectangle.fs"),
        )?;

        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));
        let position_attribute_location = context.get_attrib_location(&program, "position");
        let buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let resolution_uniform_location = context
            .get_uniform_location(&program, "resolution")
            .ok_or("failed to retrieve resolution variable from context")?;
        context.uniform2f(
            Some(&resolution_uniform_location),
            width as f32,
            height as f32,
        );
        let color_location = context
            .get_uniform_location(&program, "color")
            .ok_or("failed to get color location")?;

        let vao = context
            .create_vertex_array()
            .ok_or("failed to create a vertex array object")?;
        context.bind_vertex_array(Some(&vao));

        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            2,
            WebGl2RenderingContext::INT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);
        context.bind_vertex_array(Some(&vao));

        Ok(Webgl {
            context,
            position_attribute_location,
            resolution_uniform_location,
            color_location,
        })
    }
    pub fn draw_rectangle(&self, rect: &Rectangle) -> Result<(), JsValue> {
        let a = rect.pos.clone();
        let b = a.clone()
            + IntVec2 {
                x: rect.width,
                y: 0,
            };
        let c = a.clone()
            + IntVec2 {
                x: rect.width,
                y: rect.height,
            };
        let d = a.clone()
            + IntVec2 {
                x: 0,
                y: rect.height,
            };

        let verticies: [i32; 12] = [a.x, a.y, b.x, b.y, c.x, c.y, a.x, a.y, c.x, c.y, d.x, d.y];

        unsafe {
            let position_array_buf_view = js_sys::Int32Array::view(&verticies);
            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &position_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            )
        }
        self.context
            .uniform4fv_with_f32_array(Some(&self.color_location), rect.color.as_ref());

        let vert_count = verticies.len() as i32 / 2 as i32;
        self.draw(vert_count);

        Ok(())
    }
    pub fn clear(&self) {
        self.context.clear_color(0.0, 0.0, 0.0, 0.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
    fn draw(&self, vert_count: i32) {
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count)
    }
}

fn random_color() -> [f32; 4] {
    [
        Math::random() as f32,
        Math::random() as f32,
        Math::random() as f32,
        1.0,
    ]
}
fn random_int(max: i32) -> i32 {
    (Math::random() * max as f64) as i32 + 1 as i32
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create a shader Object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);
    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}
pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context.create_program().ok_or_else(|| {
        String::from(
            "Unable to create a shader object
  ",
        )
    })?;
    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))?
    }
}
