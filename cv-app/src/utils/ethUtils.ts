export function toEther(amount: string) {
  return +amount / Math.pow(10, 18);
}

export function txFee(gasUsed: string, gasPrice: string) {
  return +gasUsed * +gasPrice / Math.pow(10, 18);
}