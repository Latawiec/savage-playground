import fs from 'fs'
import { Image } from '@/components/game/backend/renderer/gl_resource/TextureStorage'

const ASSETS_ROOT = '/assets/TextureStorage'
const BYTE_SIZE = 8

// pngjs will always store data in internal buffer as RGBA 4x8. So until I change the library, we'll test for just that:
describe('PNG', () => {
  it.each([
    ['24bit color + alpha', __dirname + ASSETS_ROOT + '/24bit_color_alpha.png', 256, 256, false, true, 4, 8],
    ['24bit grayscale', __dirname + ASSETS_ROOT + '/24bit_grayscale.png', 256, 256, false, true, 4, 8],
    ['24bit mono dither', __dirname + ASSETS_ROOT + '/24bit_monochrome_dither.png', 256, 256, false, true, 4, 8],
    ['24bit monochrome', __dirname + ASSETS_ROOT + '/24bit_monochrome.png', 256, 256, false, true, 4, 8],
    ['256 colors', __dirname + ASSETS_ROOT + '/256colors_color.png', 256, 256, false, true, 4, 8]
  ])('Decode image: %p', async (name: string, image_file_path: string, width, height, grayscale, alpha, components, bit_depth) => {
    // Given
    const image_file_buffer = await fs.readFileSync(image_file_path)

    // When
    const image = Image.fromPNG(image_file_buffer)

    // Then
    expect(image.width).toBe(width)
    expect(image.height).toBe(height)
    expect(image.grayscale).toBe(grayscale)
    expect(image.alpha).toBe(alpha)
    expect(image.components).toBe(components)
    expect(image.bit_depth).toBe(bit_depth)
    expect(image.data.length).toBe(width * height * components * (bit_depth / BYTE_SIZE))
  })
})
