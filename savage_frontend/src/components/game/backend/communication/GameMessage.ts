
export namespace Renderer {

    export interface Assets {
        vertexShader?: string;
        pixelShader?: string;
        mesh?: string;
        textures?: Record<number, string>;
    }

    export interface VertexAttributes {
        vertices?: string;
        namedBuffers?: Record<string, string>;
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
            cameraView?: string,
            cameraProj?: string,
        },
        vec3?: {
            cameraForward?: string,
        }
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

    export interface Drawable {
        blending?: number;
        layer?: number;
        billboard?: boolean;

        assets?: Assets;
        vertexAttributes?: VertexAttributes;
        localUniformAttributes?: UniformAttributes;
        globalUniformAttributes?: GlobalUniformAttributes;
    }

    export interface Camera {
        viewTransform?: Array<number>;
        projTransform?: Array<number>;
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
        assetsPackagePath?: string
    }

    export interface Window {
        aspectRatio?: number;
        renderTargetWidth?: number;
        renderTargetHeight?: number;
    }

    export interface Snapshot {
        window?: Window;
        assets?: Assets;
    }
}

export interface GameMessage {
    renderer?: Renderer.Snapshot;
    settings?: Settings.Snapshot;
    ui?: unknown; // Each game will have it's very own UI. We'll forward this raw so that Vue Component can parse it however it wants.
}
