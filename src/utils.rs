
use glium::backend::Context;
use glium::CapabilitiesSource;

use glium::debug::{Source, MessageType, Severity};


pub fn dump_gl_info(context: &Context, is_print_extensions: bool) {
    println!("-------------------------------------------------------------");
    println!("GL Vendor    : {}", context.get_opengl_vendor_string());
    println!("GL Renderer  : {}", context.get_opengl_renderer_string());
    println!("GL Version   : {}", context.get_opengl_version_string());
    let glsl_version = context.get_supported_glsl_version();
    println!("GLSL Version : {}.{}", glsl_version.1, glsl_version.2);

    // TODO: query for GL_SAMPLES and GL_SAMPLE_BUFFERS are not implemented yet!
    // println!("MSAA samples : {}");
    // println!("MSAA buffers : {}");

    if is_print_extensions {
        let extensions = context.get_extensions();
        println!("{:?}", extensions);
    }
    println!("-------------------------------------------------------------");
}

//
// void dumpGLInfo(bool dumpExtensions) {
//     const GLubyte *renderer = glGetString( GL_RENDERER );
//     const GLubyte *vendor = glGetString( GL_VENDOR );
//     const GLubyte *version = glGetString( GL_VERSION );
//     const GLubyte *glslVersion = glGetString( GL_SHADING_LANGUAGE_VERSION );
//
//     GLint major, minor, samples, sampleBuffers;
//     glGetIntegerv(GL_MAJOR_VERSION, &major);
//     glGetIntegerv(GL_MINOR_VERSION, &minor);
// 	glGetIntegerv(GL_SAMPLES, &samples);
// 	glGetIntegerv(GL_SAMPLE_BUFFERS, &sampleBuffers);
//
// 	printf("-------------------------------------------------------------\n");
//     printf("GL Vendor    : %s\n", vendor);
//     printf("GL Renderer  : %s\n", renderer);
//     printf("GL Version   : %s\n", version);
//     printf("GL Version   : %d.%d\n", major, minor);
//     printf("GLSL Version : %s\n", glslVersion);
// 	printf("MSAA samples : %d\n", samples);
// 	printf("MSAA buffers : %d\n", sampleBuffers);
//     printf("-------------------------------------------------------------\n");
//
//     if( dumpExtensions ) {
//         GLint nExtensions;
//         glGetIntegerv(GL_NUM_EXTENSIONS, &nExtensions);
//         for( int i = 0; i < nExtensions; i++ ) {
//             printf("%s\n", glGetStringi(GL_EXTENSIONS, i));
//         }
//     }
// }


/// See https://docs.rs/glium/0.25.1/glium/debug/type.DebugCallback.html for more detail.
pub fn debug_callback(source: Source, message_type: MessageType, severity: Severity, identifier: u32, _is_handle: bool, message: &str) {

    let source_str = match source {
        Source::WindowSystem   => "WindowSys",
        Source::Application    => "Application",
        Source::Api            => "OpenGL",
        Source::ShaderCompiler => "ShaderCompiler",
        Source::ThirdParty     => "3rdParty",
        Source::OtherSource    => "Other",
    };

    let type_str = match message_type {
        MessageType::Error              => "Error",
        MessageType::DeprecatedBehavior => "Deprecated",
        MessageType::UndefinedBehavior  => "Undefined",
        MessageType::Portability        => "Portability",
        MessageType::Performance        => "Performance",
        MessageType::Marker             => "Marker",
        MessageType::PushGroup          => "PushGrp",
        MessageType::PopGroup           => "PopGrp",
        MessageType::Other              => "Other",
    };

    let severity_str = match severity {
        Severity::High         => "HIGH",
        Severity::Medium       => "MED",
        Severity::Low          => "LOW",
        Severity::Notification => "NOTIFY",
    };

    print!("{}:{}[{}]({}):{}", source_str, type_str, severity_str, identifier, message);
}


pub fn print_active_attribs(program: &glium::Program) {
    println!("-------------------------------------------------------------");
    println!("Active attributes:");
    for (name, attribute) in program.attributes() {
        println!("\tName: {:10}  Location: {}  Type: {:?}", name, attribute.location, attribute.ty);
    }
    println!("-------------------------------------------------------------");
}

pub fn print_active_uniforms(program: &glium::Program) {
    println!("-------------------------------------------------------------");
    println!("Active uniforms:");
    for (name, uniform) in program.uniforms() {
        let uniform_size = if let Some(size) = uniform.size { format!("Array({})", size) } else { String::new() };
        println!("\tName: {:10}  Location: {}  Type: {:?} {}", name, uniform.location, uniform.ty, uniform_size);
    }
    println!("-------------------------------------------------------------");
}

pub fn print_active_uniform_blocks(program: &glium::Program) {

    println!("-------------------------------------------------------------");
    println!("Active Uniform blocks:");
    for (_name, block) in program.get_uniform_blocks() {
        print_uniform_block_layout(&block.layout, 1);
    }
    println!("-------------------------------------------------------------");
}

fn print_uniform_block_layout(layout: &glium::program::BlockLayout, indent: usize) {

    use glium::program::BlockLayout;

    match layout {
        | BlockLayout::Struct { members } => {
            for (name, member) in members {
                (0..indent).for_each(|_| print!("\t"));
                println!("{}:", name);
                print_uniform_block_layout(&member, indent + 1);
            }
        },
        | BlockLayout::BasicType { ty, offset_in_buffer } => {
            (1..indent).for_each(|_| print!("\t"));
            println!("\tType: {:?},  Offset in block: {}", ty, offset_in_buffer);
        },
        | BlockLayout::Array { content, length } => {
            (1..indent).for_each(|_| print!("\t"));
            println!("\tType: Array, Length: {}", length);
            print_uniform_block_layout(&content, indent);
        },
        | _ => unimplemented!(),
    }
}
