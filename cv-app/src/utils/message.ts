
export function getTypeSummary(messages: any[]) {
  const split_message = messages[0]['@type'].split('.');
  return split_message[split_message.length - 1] + (messages.length > 1 ? ` +${messages.length - 1}` : '');
}
