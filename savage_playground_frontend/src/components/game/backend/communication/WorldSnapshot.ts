
export namespace Drawable {

    export interface Assets {
        vertex_shader?: string;
        pixel_shader?: string;
        mesh?: string;
        textures?: Map<number, string>;
    }

    export interface VertexAttributes {
        vertices?: string;
        named_buffers?: Map<string, string>;
    }

    export interface UniformAttributes {
        float?: Map<string, number>;
        vec2?: Map<string, Array<number>>;
        vec3?: Map<string, Array<number>>;
        vec4?: Map<string, Array<number>>;
        int?: Map<string, number>;
        ivec2?: Map<string, Array<number>>;
        ivec3?: Map<string, Array<number>>;
        ivec4?: Map<string, Array<number>>;
        mat4?: Map<string, Array<number>>;
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
    }
}

export interface WorldSnapshot {
    drawable?: Map<string, Drawable.Snapshot>;
    settings?: Settings.Snapshot;
}
