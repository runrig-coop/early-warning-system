export const RED = Symbol('RED');
export const YELLOW = Symbol('YELLOW');
export const GREEN = Symbol('GREEN');
export const DEFAULT_STATUS_SYMBOL = RED;

export interface StatusObject {
  symbol: symbol,
  emoji: string,
  title: string,
}

interface ColorMap {
  [color: symbol]: { title: string, emoji: string }
}
const colorMap: ColorMap = {
  [RED]: {
    emoji: '🔴',
    title: 'Red',
  },
  [YELLOW]: {
    emoji: '🟡',
    title: 'Yellow',
  },
  [GREEN]: {
    emoji: '🟢',
    title: 'Green',
  },
};

const fromSymbol = (symbol: symbol): StatusObject => {
  if (symbol in colorMap) return {
    emoji: colorMap[symbol].emoji,
    symbol,
    title: colorMap[symbol].title,
  };
  return fromSymbol(DEFAULT_STATUS_SYMBOL);
};

export const colors: StatusObject[] = Object.getOwnPropertySymbols(colorMap).map(fromSymbol);

const fromAttr = (attr?: string): StatusObject => colors.find(c =>
  attr === c.title || attr === c.emoji) || fromSymbol(DEFAULT_STATUS_SYMBOL);

export const toStatusObject = (status?: symbol|string): StatusObject =>
  typeof status === 'symbol' ? fromSymbol(status) : fromAttr(status);
