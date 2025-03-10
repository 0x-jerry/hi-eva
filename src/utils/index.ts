import { template, templateSettings } from 'lodash-es'

export function mustache(tpl: string, data: Record<string, string>) {
	templateSettings.interpolate = /{{([\s\S]+?)}}/g

	const compiled = template(tpl)

	return compiled(data)
}
