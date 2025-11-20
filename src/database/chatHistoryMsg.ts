import { type BaseModel, BaseModelManager } from './database'

export interface IChatHistoryMsgModel extends BaseModel {
  role: string
  content: string

  chatHistoryId: number
}

class ChatHistoryMsgTable extends BaseModelManager<IChatHistoryMsgModel> {
  TABLE_NAME = 'chat_history_msg'
  COLUMN_NAMES = ['role', 'content', 'chatHistoryId']

  async getMsgs(chatHistoryId: number) {
    const sql = `select * from ${this.TABLE_NAME} where chatHistoryId = $1`

    const result = await this.db.select<IChatHistoryMsgModel[]>(sql, [
      chatHistoryId,
    ])

    return result
  }
}

export const chatHistoryMsgTable = new ChatHistoryMsgTable()
