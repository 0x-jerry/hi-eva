import { bindAllMethodsToSelf } from '../utils'
import { type BaseModel, BaseModelManager } from './database'
import {
  endpointConfigTable,
  type IEndpointConfigModel,
} from './endpointConfig'
import type { GetPureModelType } from './types'

export interface IPromptConfigModel extends BaseModel {
  name?: string
  icon?: string
  prompt?: string
  isBuiltin?: number
  endpointId?: number
  orderNo?: number

  endpointConfig?: IEndpointConfigModel
}

export type IPromptConfigItem = GetPureModelType<IPromptConfigModel>

class PromptConfigTable extends BaseModelManager<IPromptConfigModel> {
  TABLE_NAME = 'prompt_config'
  COLUMN_NAMES = [
    'name',
    'icon',
    'prompt',
    'isBuiltin',
    'endpointId',
    'orderNo',
  ]

  async getById(id: number): Promise<IPromptConfigModel | undefined> {
    const resp = await super.getById(id)

    if (resp?.endpointId) {
      resp.endpointConfig = await endpointConfigTable.getById(resp.endpointId)
    }

    return resp
  }

  async getAllSorted() {
    const sorted = (await this.findAll()).sort(
      (a, b) => (a.orderNo || 0) - (b.orderNo || 0),
    )
    return sorted
  }

  async batchSaveOrder(items: IPromptConfigModel[]) {
    const stmts: string[] = []
    const params: (number | undefined)[] = []

    let idx = 1
    for (const item of items) {
      const sql = `UPDATE ${this.TABLE_NAME} SET orderNo = $${idx} WHERE id = $${idx + 1};`
      stmts.push(sql)
      params.push(item.orderNo, item.id)

      idx += 2
    }

    const sql = stmts.join('\n')

    await this.db.execute(sql, params)
  }
}

export const promptConfigTable = new PromptConfigTable()

bindAllMethodsToSelf(promptConfigTable)
