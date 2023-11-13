
export namespace Drawable {

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

    export interface UniformAttributes {
        float?: Record<string, number>;
        vec2?: Record<string, Array<number>>;
        vec3?: Record<string, Array<number>>;
        vec4?: Record<string, Array<number>>;
        int?: Record<string, number>;
        ivec2?: Record<string, Array<number>>;
        ivec3?: Record<string, Array<number>>;
        ivec4?: Record<string, Array<number>>;
        mat4?: Record<string, Array<number>>;
    }

    export interface Snapshot {
        transform?: Array<number>;
        blending?: number;
        layer?: number;
        billboard?: number;

        assets?: Assets;
        vertex_attributes?: VertexAttributes;
        uniform_attributes?: UniformAttributes;
    }
}

export namespace Settings {

    export interface Assets {
        assets_package_path?: string
    }

    export interface Camera {
        view_transform?: Array<number>;
        proj_transform?: Array<number>;
    }

    export interface Window {
        aspect_ratio?: number;
        render_target_width?: number;
        render_target_height?: number;
    }

    export interface Snapshot {
        camera?: Camera;
        window?: Window;
        assets?: Assets;
    }
}

export interface GameMessage {
    drawable?: Record<string, Drawable.Snapshot>;
    settings?: Settings.Snapshot;
}
