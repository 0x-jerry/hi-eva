import { bindAllMethodsToSelf } from '../utils'
import {
  chatHistoryMsgTable,
  type IChatHistoryMsgModel,
} from './chatHistoryMsg'
import { type BaseModel, BaseModelManager } from './database'

export interface IChatHistoryModel extends BaseModel {
  name: string

  messages?: IChatHistoryMsgModel[]
}

class ChatHistoryTable extends BaseModelManager<IChatHistoryModel> {
  TABLE_NAME = 'chat_history'
  COLUMN_NAMES = ['name']

  async getAllById(id: number) {
    const result = await this.getById(id)

    if (result) {
      const msgs = await chatHistoryMsgTable.getMsgs(result.id)

      result.messages = msgs
    }

    return result
  }
}

export const chatHistoryTable = new ChatHistoryTable()

bindAllMethodsToSelf(chatHistoryTable)
