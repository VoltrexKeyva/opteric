interface FlagsAndOptions {
  flags: string[];
  options: Record<string, string>;
  content: string;
};

function parse(text: string): FlagsAndOptions {
  const flags: string[] = [],
    options: Record<string, string> = {},
    len: number = text.length + 1;
  let content: string;

  for (
    let i = 0,
      parsing: string = '',
      isParsing: boolean = false,
      optContent: string = '',
      isParsingContent = false;
    i < len;
    i++
  ) {
    const char: string = text[i],
      charPrev: string = text[i - 1],
      charNext: string = text[i + 1];

    if (isParsing) {
      if (char === ' ' || char === undefined) {
        isParsing = false;

        if (charNext !== ' ' && charNext !== '-' && charNext !== undefined) {
          isParsingContent = true;

          continue;
        }

        flags.push(parsing);

        parsing = '';

        if (content[0] === '-') content = '';
      } else parsing += char;
    } else if (isParsingContent) {
      if ((char === ' ' && charNext === '-') || char === undefined) {
        isParsingContent = false;
        options[parsing] = optContent;
        parsing = '';
        optContent = '';
      } else optContent += char;
    } else if ((charPrev === ' ' || charPrev === undefined) && char === '-') {
      const truncateLen = i - 1;

      i++;
      isParsing = true;

      if (charNext === '-') i++;

      if (content === undefined)
        content = charPrev === undefined ? text : text.slice(0, truncateLen);

      parsing += text[i];
    }
  }

  if (content === undefined) content = text;

  return { flags, options, content };
}

export default parse;
export { FlagsAndOptions };