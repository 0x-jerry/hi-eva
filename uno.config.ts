import {
	defineConfig,
	presetIcons,
	presetWind3,
	transformerDirectives,
	transformerVariantGroup,
} from 'unocss'
import process from 'node:process'

const isDev = process.env.NODE_ENV === 'development'

const carbonIcons = isDev ? [] : await getCarbonIcons()

async function getCarbonIcons() {
	const icons = (await import('@iconify-json/carbon')).icons

	return Object.keys(icons).map((name) => `i-${icons.prefix}:${name}`)
}

export default defineConfig({
	presets: [presetWind3(), presetIcons()],
	transformers: [transformerVariantGroup(), transformerDirectives()],
	safelist: [...carbonIcons],
})
