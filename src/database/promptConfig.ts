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

  endpointConfig?: IEndpointConfigModel
}

export type IPromptConfigItem = GetPureModelType<IPromptConfigModel>

class PromptConfigTable extends BaseModelManager<IPromptConfigModel> {
  TABLE_NAME = 'prompt_config'
  COLUMN_NAMES = ['name', 'icon', 'prompt', 'isBuiltin', 'endpointId']

  async getById(id: number): Promise<IPromptConfigModel | undefined> {
    const resp = await super.getById(id)

    if (resp?.endpointId) {
      resp.endpointConfig = await endpointConfigTable.getById(resp.endpointId)
    }

    return resp
  }
}

export const promptConfigTable = new PromptConfigTable()
