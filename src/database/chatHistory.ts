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

  /**
   * Delete all chat history messages
   * @param id
   */
  async deleteAllById(id: number) {
    await this.deleteById(id)

    const sql = `delete from ${chatHistoryMsgTable.TABLE_NAME} where chatHistoryId = $1`

    await this.db.execute(sql, [id])
  }
}

export const chatHistoryTable = new ChatHistoryTable()

bindAllMethodsToSelf(chatHistoryTable)
