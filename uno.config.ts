import {
	defineConfig,
	presetIcons,
	presetWind3,
	transformerDirectives,
	transformerVariantGroup,
} from 'unocss'
import { icons } from '@iconify-json/carbon'
import process from 'node:process'

const isDev = process.env.NODE_ENV === 'development'

const carbonIcons = isDev
	? ['i-carbon:ibm-watson-language-translator', 'i-carbon:book']
	: Object.keys(icons.icons).map((name) => `i-${icons.prefix}:${name}`)

export default defineConfig({
	presets: [presetWind3(), presetIcons()],
	transformers: [transformerVariantGroup(), transformerDirectives()],
	safelist: [...carbonIcons],
})
