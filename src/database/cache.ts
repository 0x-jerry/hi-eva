import { bindAllMethodsToSelf } from '../utils'
import { type BaseModel, BaseModelManager } from './database'
import type { GetPureModelType } from './types'

export interface ICacheModel extends BaseModel {
  key: string
  value: string

  /**
   * Time range in seconds
   */
  expiredIn?: number
}

export type ICacheItem = GetPureModelType<ICacheModel>

class CacheTable extends BaseModelManager<ICacheModel> {
  TABLE_NAME = 'cache'
  COLUMN_NAMES = ['key', 'value', 'expiredIn']

  async getByKey(key: string) {
    const sql = `select * from ${this.TABLE_NAME} where key = $1`

    const resp = await this.db.select<ICacheModel[]>(sql, [key])

    return resp.at(0)
  }

  async upsertByKey(data: ICacheItem) {
    const exists = await this.getByKey(data.key)

    if (exists) {
      await this.updateOne({
        ...data,
        id: exists.id,
      })
    } else {
      await this.createOne(data)
    }
  }
}

export const cacheTable = new CacheTable()

bindAllMethodsToSelf(cacheTable)
