import { CommitedResourceStorage } from "./gl_resource/CommitedResourceStorage";
import { BackBufferTarget } from "./pipeline/render_target/BackBufferTarget";
import { MainTarget } from "./pipeline/render_target/MainTarget";
import { IDrawCommand } from "./pipeline/command/IDrawCommand";
import { ZipAssetStorage } from "../assets/ZipAssetStorage";
import { LocalAssetStorage } from "../assets/LocalAssetStorage";
import { IAssetStorage } from "../assets/IAssetStorage";
import { PresentDrawCommand } from "./pipeline/command/PresentDrawCommand";

export class GameRenderer {
    private _gl: WebGLRenderingContext;
    private _external_resources_storage: CommitedResourceStorage;
    private _local_resources_storage: CommitedResourceStorage;
    private _back_buffer_render_target: BackBufferTarget;
    private _main_render_target: MainTarget;

    constructor(
        gl: WebGLRenderingContext
    ) {
        this._gl = gl;
        this._external_resources_storage = new CommitedResourceStorage(this._gl, ZipAssetStorage.from_empty());
        this._local_resources_storage = new CommitedResourceStorage(this._gl, new LocalAssetStorage());

        // Just some default values. I expect it to be overwritten anyways.
        this._back_buffer_render_target = new BackBufferTarget(this._gl, 100, 100);
        this._main_render_target = new MainTarget(this._gl, 100, 100);
    }

    set external_asset_storage(asset_storage: IAssetStorage) {
        this._external_resources_storage = new CommitedResourceStorage(this._gl, asset_storage);
    }

    get external_resource_storage(): CommitedResourceStorage {
        return this._external_resources_storage;
    }

    get local_resource_storage(): CommitedResourceStorage {
        return this._local_resources_storage;
    }

    execute_draw_commands(draw_commands: IDrawCommand[]): Promise<void> {
        return new Promise(async (resolve) => {
            const gl = this._gl;

            this._back_buffer_render_target.bind();
            gl.clearColor(0.0, 0.0, 0.0, 1.0);
            gl.clearDepth(1.0);
            gl.disable(gl.DEPTH_TEST);
            gl.enable(gl.BLEND);

            gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

            for (const draw_command of draw_commands) {
                draw_command.draw(this._gl);
            }
            resolve();
        })
    }

    present(): Promise<void> {
        return new Promise(async (resolve) => {
            const gl = this._gl;
            this._main_render_target.bind();

            const draw_command = await PresentDrawCommand.from_resources(this._local_resources_storage, this._back_buffer_render_target.color_texture);
            draw_command.draw(gl);
            resolve();
        })
    }

    resize_buffers(width: number, height: number) {
        this._back_buffer_render_target = new BackBufferTarget(this._gl, width, height);
        this._main_render_target = new MainTarget(this._gl, width, height);
    }
}
