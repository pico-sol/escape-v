# escape-v

プロジェクトvに預け入れたUSDCを引き出すtxを10秒毎に繰り返すRustプログラムです

mainの20-27行目とid.jsonの中身を設定して使用してください

以下のような表示が繰り返すのは正常な状態です

vaultka balance: 0.0 USDC
acc1: EYTN9eRR4y4zN2yCR9L8cWvvbWbGTSuNrRT1ixMf6wND
acc2: CfkxTsptuC7eWT4Hbs9o6QxN4cViaM9HUf3HxcCtPxPt
acc3: F2kJ9Mx6d7KkJogbLeJC7ir4td2heyDPa7qvY3kzyqpa
acc4: 11111111111111111111111111111111
acc5: HpVZ5RAK1p4FLhQxBCLi8HGeitbCmBQhSYexW2ngU4eX
acc6: E19zKjNZhWhvHfHao89Pk9xQ2zSr6DErGSaFnddyuY3A
acc7: 8QFAHBZFkWu6mZVo3YyZgUWcRtaN4bv43UFG2awrvw7h
acc8: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
acc9: DefkwTSvkHeZASCuaVJ8AxUWS6zvBCwrLFpW2FniLSWo
acc10: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
error: Error { request: Some(SendTransaction), kind: RpcError(RpcResponseError { code: -32002, message: "Transaction simulation failed: Error processing Instruction 2: custom program error: 0x1771", data: SendTransactionPreflightFailure(RpcSimulateTransactionResult { err: Some(InstructionError(2, Custom(6001))), logs: Some(["Program ComputeBudget111111111111111111111111111111 invoke [1]", "Program ComputeBudget111111111111111111111111111111 success", "Program ComputeBudget111111111111111111111111111111 invoke [1]", "Program ComputeBudget111111111111111111111111111111 success", "Program nKMLJtN1rr64K9DjmfzXvzaq4JEy5a4AJHHP9gY1dW6 invoke [1]", "Program log: Instruction: Withdraw", "Program log: Max withdraw: 2000241", "Program log: Lending balance: 0", "Program log: AnchorError thrown in programs/waterusdc/src/lib.rs:596. Error Code: InvalidWithdrawAmount. Error Number: 6001. Error Message: Withdraw amount must be an amount available in the lending_account.", "Program nKMLJtN1rr64K9DjmfzXvzaq4JEy5a4AJHHP9gY1dW6 consumed 18609 of 199700 compute units", "Program nKMLJtN1rr64K9DjmfzXvzaq4JEy5a4AJHHP9gY1dW6 failed: custom program error: 0x1771"]), accounts: None, units_consumed: Some(18909), return_data: None, inner_instructions: None }) }) }
