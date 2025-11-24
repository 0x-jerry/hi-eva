import OpenAI from 'openai'
import { createCacheableFn } from './cache'

async function _fetchModelList(opt: { baseUrl: string; apiKey?: string }) {
  const { baseUrl, apiKey } = opt

  if (!apiKey) {
    return []
  }

  const ins = new OpenAI({
    dangerouslyAllowBrowser: true,
    baseURL: baseUrl,
    apiKey,
  })

  const list = await ins.models.list()

  return list.data.map((n) => ({
    label: n.id,
    value: n.id,
  }))
}

const fetchModelList = createCacheableFn(_fetchModelList, {
  getKey(opt) {
    return opt.baseUrl
  },
})

export { fetchModelList }
