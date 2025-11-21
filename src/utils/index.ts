import { template, templateSettings } from 'lodash-es'

export function mustache(tpl: string, data: Record<string, string>) {
  templateSettings.interpolate = /{{([\s\S]+?)}}/g

  const compiled = template(tpl)

  return compiled(data)
}

export function bindAllMethodsToSelf(obj: Record<string, any>) {
  for (const key in obj) {
    if (typeof obj[key] === 'function') {
      obj[key] = obj[key].bind(obj)
    }
  }
}
