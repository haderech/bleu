export function getSimpleType(type: string) {
  const split_type = type.split('.');
  return split_type[split_type.length - 1];
}

export function getSimpleStatus(status: string) {
  const split_status = status.split('_');
  return split_status[split_status.length - 1];
}

export function getTypeSummary(messages: any[]) {
  const split_message = messages[0]['@type'].split('.');
  return split_message[split_message.length - 1] + (messages.length > 1 ? ` +${messages.length - 1}` : '');
}
