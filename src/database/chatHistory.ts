import { chatHistoryMsgTable, IChatHistoryMsgModel } from './chatHistoryMsg'
import { type BaseModel, BaseModelManager } from './database'
import { IPromptConfigModel } from './promptConfig'

export interface IChatHistoryModel extends BaseModel {
  title: string
  model: string

  promptConfigId: number
  promptConfig?: IPromptConfigModel

  messages?: IChatHistoryMsgModel[]
}

class ChatHistoryTable extends BaseModelManager<IChatHistoryModel> {
  TABLE_NAME = 'chat_history'
  COLUMN_NAMES = ['title', 'model', 'promptConfigId']

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
