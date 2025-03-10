import {
	defineConfig,
	presetIcons,
	presetWind3,
	transformerDirectives,
	transformerVariantGroup,
} from 'unocss'
import { icons } from '@iconify-json/carbon'

const carbonIcons = Object.keys(icons.icons).map(
	(name) => `i-${icons.prefix}:${name}`,
)

export default defineConfig({
	presets: [presetWind3(), presetIcons()],
	transformers: [transformerVariantGroup(), transformerDirectives()],
	safelist: [...carbonIcons],
})
