import type { Fn } from '@0x-jerry/utils'
import { cacheTable } from '../database/cache'

async function getCache<T>(key: string) {
  const result = await cacheTable.getByKey(key)

  if (result?.value == null) {
    return
  }

  if (result.expiredIn) {
    const isExpired = Date.now() > result.createdDate + result.expiredIn * 1000

    if (isExpired) {
      await cacheTable.deleteById(result.id)
      return
    }
  }

  return JSON.parse(result.value) as T
}

export type ICacheableFnReturn<T extends Fn> = T & { forceUpdate: T }

export function createCacheableFn<T extends Fn>(
  fn: T,
  config: {
    getKey: (...args: Parameters<T>) => string
  },
) {
  async function wrapperFn(...args: Parameters<T>) {
    const key = config.getKey(...args)

    const hitCache = await getCache(key)

    if (hitCache !== undefined) {
      return hitCache
    }

    return updateCache()

    async function updateCache() {
      const result = await fn(...args)

      cacheTable.upsertByKey({
        key,
        value: JSON.stringify(result),
      })

      return result
    }
  }

  wrapperFn.forceUpdate = async (...args: Parameters<T>) => {
    const key = config.getKey(...args)

    return updateCache()

    async function updateCache() {
      const result = await fn(...args)

      cacheTable.upsertByKey({
        key,
        value: JSON.stringify(result),
      })

      return result
    }
  }

  return wrapperFn as ICacheableFnReturn<T>
}
