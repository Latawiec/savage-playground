
export namespace Renderer {

    export interface Assets {
        vertex_shader?: string;
        pixel_shader?: string;
        mesh?: string;
        textures?: Record<number, string>;
    }

    export interface VertexAttributes {
        vertices?: string;
        named_buffers?: Record<string, string>;
    }

    // Predefined global attributes. Not mandatory.
    // Values for these will be set from the renderer so no need to pass them here.
    // Just pass in the names where you want these values to be parked at.
    //
    // You can overwrite the values of any of these
    // by simply providing the value as non-global field in UniformAttributes.
    // Or just don't put the names of them here.
    export interface GlobalUniformAttributes {
        mat4?: {
            camera_view?: string,
            camera_proj?: string,
        },
        vec3?: {
            camera_forward?: string,
        }
    }

    export interface UniformAttributes {
        float?: Record<string, number>;
        vec2?:  Record<string, Array<number>>;
        vec3?:  Record<string, Array<number>>;
        vec4?:  Record<string, Array<number>>;
        int?:   Record<string, number>;
        ivec2?: Record<string, Array<number>>;
        ivec3?: Record<string, Array<number>>;
        ivec4?: Record<string, Array<number>>;
        mat4?:  Record<string, Array<number>>;
    }

    export interface Drawable {
        transform?: Array<number>;
        blending?: number;
        layer?: number;
        billboard?: boolean;

        assets?: Assets;
        vertex_attributes?: VertexAttributes;
        local_uniform_attributes?: UniformAttributes;
        global_uniform_attributes?: GlobalUniformAttributes;
    }

    export interface Camera {
        view_transform?: Array<number>;
        proj_transform?: Array<number>;
    }

    export interface Snapshot {
        // Increment: add/remove elements from whatever was already passed before.
        //            It assumes state was being cached.
        // Reset: erase whole state you've had before and replace it with this.
        // If nothing is passed, we assume 'reset'
        type?: 'increment' | 'reset';
        entities?: Record<string, Drawable>;
        camera?: Camera;
    }
}

export namespace Settings {

    export interface Assets {
        assets_package_path?: string
    }

    export interface Window {
        aspect_ratio?: number;
        render_target_width?: number;
        render_target_height?: number;
    }

    export interface Snapshot {
        window?: Window;
        assets?: Assets;
    }
}

export interface GameMessage {
    renderer?: Renderer.Snapshot;
    settings?: Settings.Snapshot;
    ui?: any; // Each game will have it's very own UI. We'll forward this raw so that Vue Component can parse it however it wants.
}
