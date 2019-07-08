
use glium::backend::Context;
use glium::CapabilitiesSource;


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
