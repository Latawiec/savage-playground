import fs from 'fs'
import { Image } from '@/components/game/backend/renderer/gl_resource/TextureStorage'
import path from 'path'

const ASSETS_ROOT = '/assets/TextureStorage'
const BYTE_SIZE = 8

// pngjs will always store data in internal buffer as RGBA 4x8. So until I change the library, we'll test for just that:
describe('PNG', () => {
  it.each([
    ['24bit color + alpha', path.join(__dirname, ASSETS_ROOT, '/24bit_color_alpha.png'), 256, 256, false, true, 4, 8],
    ['24bit grayscale', path.join(__dirname, ASSETS_ROOT, '/24bit_grayscale.png'), 256, 256, false, true, 4, 8],
    ['24bit mono dither', path.join(__dirname, ASSETS_ROOT, '/24bit_monochrome_dither.png'), 256, 256, false, true, 4, 8],
    ['24bit monochrome', path.join(__dirname, ASSETS_ROOT, '/24bit_monochrome.png'), 256, 256, false, true, 4, 8],
    ['256 colors', path.join(__dirname, ASSETS_ROOT, '/256colors_color.png'), 256, 256, false, true, 4, 8]
  ])('Decode image: %p', async (name: string, imageFilePath: string, width, height, grayscale, alpha, components, bitDepth) => {
    // Given
    const imageFileBuffer = fs.readFileSync(imageFilePath)

    // When
    const image = Image.fromPNG(imageFileBuffer)

    // Then
    expect(image.width).toBe(width)
    expect(image.height).toBe(height)
    expect(image.grayscale).toBe(grayscale)
    expect(image.alpha).toBe(alpha)
    expect(image.components).toBe(components)
    expect(image.bitDepth).toBe(bitDepth)
    expect(image.data.length).toBe(width * height * components * (bitDepth / BYTE_SIZE))
  })
})
