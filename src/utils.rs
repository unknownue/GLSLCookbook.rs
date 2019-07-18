
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
        Source::WindowSystem   => String::from("WindowSys"),
        Source::Application    => String::from("Application"),
        Source::Api            => String::from("OpenGL"),
        Source::ShaderCompiler => String::from("ShaderCompiler"),
        Source::ThirdParty     => String::from("3rdParty"),
        Source::OtherSource    => String::from("Other"),
    };

    let type_str = match message_type {
        MessageType::Error              => String::from("Error"),
        MessageType::DeprecatedBehavior => String::from("Deprecated"),
        MessageType::UndefinedBehavior  => String::from("Undefined"),
        MessageType::Portability        => String::from("Portability"),
        MessageType::Performance        => String::from("Performance"),
        MessageType::Marker             => String::from("Marker"),
        MessageType::PushGroup          => String::from("PushGrp"),
        MessageType::PopGroup           => String::from("PopGrp"),
        MessageType::Other              => String::from("Other"),
    };

    let severity_str = match severity {
        Severity::High => String::from("HIGH"),
        Severity::Medium => String::from("MED"),
        Severity::Low => String::from("LOW"),
        Severity::Notification => String::from("NOTIFY"),
    };

    eprint!("{}:{}[{}]({}):{}", source_str, type_str, severity_str, identifier, message);
}


pub fn print_active_attribs(program: &glium::Program) {

    println!("Active attributes:");
    for (name, attribute) in program.attributes() {
        println!("\tName: {}, Location: {} - Type: {:?}", name, attribute.location, attribute.ty);
    }
}
