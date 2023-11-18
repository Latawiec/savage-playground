import { PNG } from 'pngjs'
import { IAssetStorage } from '../../assets/IAssetStorage'

type Components = 1 | 2 | 3 | 4; // Grayscale: 1, Gayscale+Alpha: 2 etc...
type BitDepth = 1 | 2 | 4 | 8 | 16; // How many bits per one component. I don't know if less than 8 is realistic.

export class Image {
    data: Uint8Array;
    width: number;
    height: number;
    grayscale: boolean;
    alpha: boolean;
    components: Components;
    bitDepth: BitDepth;

    constructor (
      imageData: Uint8Array,
      width: number,
      height: number,
      grayscale: boolean,
      alpha: boolean,
      components: Components,
      bitDepth: BitDepth
    ) {
      this.data = imageData
      this.width = width
      this.height = height
      this.grayscale = grayscale
      this.alpha = alpha
      this.components = components
      this.bitDepth = bitDepth
    }

    static fromPNG (pngFileData: Readonly<Buffer>): Image {
      const png = PNG.sync.read(pngFileData)

      // buffer from PNG object will always store data as RGBA 4x8. So we'll keep it this way.
      return new Image(
        png.data,
        png.width,
        png.height,
        false, // grayscale
        true, // alpha
        4, // components
        8 // bit_depth
      )
    }
}

export class Texture {
    private _texture: WebGLTexture;

    constructor (glContext: WebGLRenderingContext, textureImage: Image) {
      const texture = glContext.createTexture()
      if (!texture) {
        throw new Error('Couldn\'t create texture')
      }
      this._texture = texture

      // TODO: I should parametrize these in construction. But right now wioth pngjs only, all images will be the same format.
      glContext.bindTexture(glContext.TEXTURE_2D, this._texture)
      glContext.texImage2D(glContext.TEXTURE_2D, 0, glContext.RGBA, textureImage.width, textureImage.height, 0, glContext.RGBA, glContext.UNSIGNED_BYTE, textureImage.data)
      glContext.texParameteri(glContext.TEXTURE_2D, glContext.TEXTURE_MAG_FILTER, glContext.NEAREST)
      glContext.texParameteri(glContext.TEXTURE_2D, glContext.TEXTURE_MIN_FILTER, glContext.NEAREST)
      glContext.texParameteri(glContext.TEXTURE_2D, glContext.TEXTURE_WRAP_S, glContext.CLAMP_TO_EDGE)
      glContext.texParameteri(glContext.TEXTURE_2D, glContext.TEXTURE_WRAP_T, glContext.CLAMP_TO_EDGE)
    }

    get glTexture (): Readonly<WebGLTexture> { return this._texture }
}

export class TextureStorage {
    private _textureCache = new Map<string, Texture>();

    private _assetStorage: IAssetStorage;
    private _gl: WebGLRenderingContext;

    constructor (gl: WebGLRenderingContext, assetStorage: IAssetStorage) {
      this._assetStorage = assetStorage
      this._gl = gl
    }

    write (assetPath: string, texture: Texture) {
      if (this._textureCache.has(assetPath)) {
        throw new Error(`Asset with path ${assetPath} already exists in ${TextureStorage.name}`)
      }
      this._textureCache.set(assetPath, texture)
    }

    async read (assetPath: string): Promise<Texture> {
      const texture = this._textureCache.get(assetPath)
      if (texture) {
        return texture
      }

      try {
        // For now we'll assume all images are PNG.
        const pngFileData = (await this._assetStorage.readFile(assetPath)) as Buffer
        const image = Image.fromPNG(pngFileData)

        const texture = new Texture(this._gl, image)
        this._textureCache.set(assetPath, texture)

        return texture
      } catch (e) {
        throw new Error(`Couldn't retrieve texture ${assetPath}: ${e}`)
      }
    }
}
