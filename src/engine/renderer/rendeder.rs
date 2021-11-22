

pub mod pe_renderer{
    use vulkano::instance::Instance;
    use vulkano::instance::InstanceExtensions;
    use vulkano::instance::PhysicalDevice;

    use vulkano::device::Device;
    use vulkano::device::DeviceExtensions;
    use vulkano::device::Features;


    use vulkano::buffer::BufferUsage;
    use vulkano::buffer::CpuAccessibleBuffer;


    use vulkano::command_buffer::AutoCommandBufferBuilder; 

    use vulkano::framebuffer::Framebuffer;

    use vulkano::format::Format;
   
    use vulkano::image::Dimensions;
    use vulkano::image::StorageImage;

    use std::sync::Arc;
    
    use vulkano::pipeline::GraphicsPipeline;
    use vulkano::framebuffer::Subpass;
    
    use vulkano::command_buffer::DynamicState;
    use vulkano::pipeline::viewport::Viewport;



    pub fn pe_renderer_init(){
        let instance = Instance::new(None, &InstanceExtensions::none(), None)
            .expect("failed to create instance"); 
        


        let physical = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("no device available");

        
        for family in physical.queue_families() {
            println!("Found a queue family with {:?} queue(s)",
                    family.queues_count());
        }


        let queue_family = physical.queue_families()
            .find(|&q| q.supports_graphics())
            .expect("couldn't find a graphical queue family");

        let (device, mut queues) = {
                Device::new(physical, &Features::none(), &DeviceExtensions::none(),
                 [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
            };      
        

        let queue = queues.next().unwrap();
        

let render_pass = Arc::new(vulkano::single_pass_renderpass!(device.clone(),
    attachments: {
        color: {
            load: Clear,
            store: Store,
            format: Format::R8G8B8A8Unorm,
            samples: 1,
        }
    },
    pass: {
        color: [color],
        depth_stencil: {}
    }
).unwrap());

let image = StorageImage::new(device.clone(), Dimensions::Dim2d { width: 1024, height: 1024 },
                              Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();


let framebuffer = Arc::new(Framebuffer::start(render_pass.clone())
    .add(image.clone()).unwrap()
    .build().unwrap());




        println!("Vulkan Renderer init");
    }
}
