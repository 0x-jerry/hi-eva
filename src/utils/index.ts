import { template, templateSettings } from 'lodash-es'

export function mustache(tpl: string, data: Record<string, string>) {
  templateSettings.interpolate = /{{([\s\S]+?)}}/g

  const compiled = template(tpl)

  return compiled(data)
}

// biome-ignore lint/suspicious/noExplicitAny: on purpose
export function bindAllMethodsToSelf(obj: Record<string, any>, target = obj) {
  const keys = Object.getOwnPropertyNames(obj)

  const excludeKeys = ['constructor']

  for (const key of keys) {
    if (excludeKeys.includes(key)) {
      continue
    }

    if (typeof target[key] === 'function') {
      target[key] = target[key].bind(target)
    }
  }

  const superProto = Object.getPrototypeOf(obj)
  if (superProto !== Object.prototype) {
    bindAllMethodsToSelf(superProto, target)
  }
}
