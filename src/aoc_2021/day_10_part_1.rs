use std::collections::HashMap;

#[allow(dead_code)]
fn day_10_part_1(input: &str) -> i64 {
    let opening_chars = vec!['(', '[', '{', '<'];
    let closing_chars = vec![')', ']', '}', '>'];
    let char_pairs = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>')
    ]);
    let char_scores = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    let is_opening_char = |ch| {
        return opening_chars.iter().find(|&&open_ch| open_ch == ch) != None;
    };
    let is_closing_char = |ch| {
        return closing_chars.iter().find(|&&open_ch| open_ch == ch) != None;
    };
    let get_matching_closing_char = |ch| {
        return *char_pairs.get(&ch).unwrap();
    };

    let mut illegal_closing_chars = Vec::<char>::new();

    for line in input.split('\n') {
        let mut stack = Vec::<char>::new();
        for ch in line.chars() {
            if is_opening_char(ch) {
                stack.push(ch);
            }
            else if is_closing_char(ch) {
                if stack.is_empty() {
                    illegal_closing_chars.push(ch);
                    break;
                }
                let opening_char = *stack.last().unwrap();
                let correct_closing_char = get_matching_closing_char(opening_char);
                if ch != correct_closing_char {
                    illegal_closing_chars.push(ch);
                    break;
                }
                stack.pop();
            }
        }
    }

    let ans = illegal_closing_chars.iter().map(|ch| char_scores.get(ch).unwrap()).sum();

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_10_part_1() {
        let input =
r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

        let ans = day_10_part_1(&input);

        assert_eq!(ans, 26397);
    }

    #[test]
    fn test_day_10_part_1() {
        let input =
r#"[<<{[{([([({<{<>{}}>[[[]{}][[]<>])}{[(())][[<>{}]{[]{}}]})](([{<<>{}>[()<>]}<<<>()>{{}<>}>]{[{{}()}[(){}]]})
[({[[(<{[<[<{({}[])(<>{})}(([]<>)[[]()])>{[<{}()><{}{}>][<[]()>({}())]}]<(<(()<>){{}{}}>({()()}(()[])))>>[([
(<([[<(<<{((<<<>{}>{{}{}}>({()<>}{()<>}))<[(<><>)<<>()>]{((){}){[]<>}}>)}({<[(()[]){()[]}]{((
<[(<{[{{([([{{{}<>}}{{<>()}<<>>}]){[[<[]<>>(())]<<<>{}>(())>][<<{}{}>(<>())>{<{}{}><{}<>>}]}]<<({{()
{(((<{{{<<{({{()[]){()[]}}({{}}[<>[]])){[{()<>}<()()>]}}>{[[{<[]{}>{{}()}}<<()[]>[{}[]]>]{[{(){}}(()[])]<({
<(<{<<{{([[({{<>()}[(){}>}[(<>)(()[])])([[<>()]<[]()>])]{(<(<>()){[][]}>[((){}){[]{}}])[[{[]<>}({}
((<[<(([[{[{{[{}()]{<>[]}}}]{[{<()()>[{}[]]}[<[]{}>]]{{{<><>}[()[]]}<{{}{}}[<>()]>}}}[{{<{[][]}({}())><{
[([{({[({<[[<([][])<(){}>><<<><>>[[]()]>]<<({}{}))[<<>[]>]>]{[(([]<>){[]<>}){[{}{}](()())}]}>[<<{<[][
({{{(<{({((<({<>()})[(<>[])<{}()>]>{{<[]()><{}[])}[{[]{}}({}{})]})){{{[<(){}><[]<>>][<{}()>{(){}
(<[<[<{[<{{[[([]{})]](([{}{}]({}[])){<()()>({}())})}[((<{}()>{[][]}))[[<()[]><()[]>>([[]<>]([]{}))]]}({[
<{{[<[<<(<[[([[]<>]({}<>))][[({}<>){{}()}]{<(){}>(()[])}]]>{[([[[]{}][{}()]][({}{})((){})])({({}
([{{[{{[<<[([([]())<[][]>]({<>()}{[]()}))<(<<>{}>{()})>][[[{<>[]}[(){}]][[()[]]({}())]][(<[]{}
<[{<[<[<{[<([[{}()]]){<{{}<>}[{}<>]>([{}{}]<[][]>)}>][<([{(){}}<<><>>]<<[]{}>[()<>]>)><{[([](
<([[([((<[{<<<()<>><[][]>>{({}[])({}{})}>({({}<>){[]}})}<(<{[]{}}{()<>}>([(){}][()<>]))[{[{}]<{}()>}<{{}
[{{[([{{{{(({({}<>)<[][]>})([{<>()]<<>{}>]{[()()]}))({{<(){}>{<>{}}}}([<()<>>[{}{}]]{{[]{}}(()[])}))}{(((<[]
<{{{([<[{((<<<[]{}>{()[]}>{(()[])[{}()]}>[[(<><>)][[(){}]{[]<>}]])({{((){})<[][]>}[<[]()>(<>{})]}<([{}<
<[(((<{(({[{[[[]()][[]<>]]<{<><>}[()()]>}][{[[{}{}]<{}()>]<([]()){<>[])>}[(<{}<>>({}()))(<<><>>((){}
([{{[[((<{([<<()[]>{[][]}>{[[]{}]<{}()>]]{<<()<>>>{({}<>)((){})}})<[{[<>()](<>[])}]{({()}({}
(({[{(({{{<{(({}{}){[]<>})<{[][]}>}<<{{}[]}<{}}>{{()[]}{<>[]}}>>}(<<[([]{})<<>()>]<(()<>)<{}<>>>>>[{{(()[]){{
<[{<<<(([{{(<[[]()][{}[]]>[[{}()]<{}()>])<[[[]]{[][]}][(()[]}]>}}][{({<<<><>>>[<{}[]>({}{})]}<(([])<{}{}>)>)
{<(<[(({<[<{(({}){()<>}}}([[<>()](()[])])>((<[[]{}]{[]<>}>({[]<>}[[]<>])))]>}({{{[{{<>()}[(){
[{<{{{[<[[<<<[<>]>[[[][]][()<>]]>{<{(){}}<()[]>>([()[]](<>))}>[{[{<>{}}[[][]]][[[][]][<>[]])}<({{}()}{(){}}){
<<<<(([((<(([{<>()}<[]{}>]))({({{}[]}<(){}>)[(()())<{}<>>]}<{<[]()>{()[]}}([<><>]([]{}))>)>[<{<<()[]>[()<>
<{<{<((({[<{(<{}()><()[]>)<[{}()]<<>()]>}<{(<>())[[]<>]}[(<>[])[<>[]]]>>]<<<<([]{})(()())>{({}(
(<[(({[(<{(([({}{})(<><>)]<{[]}(())>)<([(){}])>)(<(((){})<<>])([{}{}][{}{}])><[<()()>[{}()]]{((
<{<(<([<{[<{[{<>()}{<>[]}]}<<[[]()]({}<>)><[[][]]([]{})>>>[[[{<>{}}(()())]({()<>}<{}{}>)]]
{[<[(<((<{<[[<[]()>[[]()]]([{}()]<<>>)]([([][])[(){}]])><{({<>[]}[[]<>])<{<>()}<()()>>}((<{}()>)<(<>[]){[]()
<{<[{((<[{([[<<><>>(()<>)](([]()){[]()})]<{(<><>){(){}}}>)}<(([{()()}<<>[]>])(<<<>{}>(()())><<{}{}><{}[]>>
(<([{<<[<({({[<>{}](<>{})}((<>{})<{}{}>))[<[()](<>())>{[[]()]<()<>>}]})[{({(()[]){[]{}}}[[[]()]
{({<(<{{[<(<([<>[]][{}{}])[[{}<>]{[][]}]>[[((){})[<>()]]({[]<>}<{}()>)]){[([[][]]<[]{}>)[[{}()]]][[([][]){[
({[<{({{(<{[{<{}[]>{<><>}}<<<>[]>([]())>](<(<><>)<[]{}>><{<>()}<()<>>>]}<{[([])<{}<>>][<[]<>>
[{<[<{<({{(((<{}[]>{[]{}}){<<>[]>[<>[]]}))<[(<<>[]>{[]{}})[<{}[]>{<>{}}]]>}{([{[<>{}]([][])}<{()()}{<><>
{[([{<([{{<[<{(){}}[()<>]><[[]][[]<>]>]([<()<>>({}<>)][<{}()>[<><>]])><<{<()()><<>()>}[[<>[]
<{({[{{([({(({[]{}}<{}{}>)({[][]}<[][]>)){{[()<>]<<><>>}{<{}{}>}}}{({[[]()](<>())}{[()[]]}>[<{()}{[]
<{<({([{{{((<{<>}<[]()>><[()()][[]<>]>)([{()<>}{<>[]}](<<>()>([]{})]))(<{<(){}>}([<>[]](()[]))>[
[{[({({<(([({{{}<>}}[<[]<>]({}{})]){({()()}{{}[]})}]{[[[()<>]<<>[]>][([][])]]})<[<<([]<>)([]{})><{<
<[[[({{{([{({<()[]>(<><>)}<{<>}([]())>)([[{}[]]<<>()>]([<>()][[]<>]))}<{<(()[])>>>]<<<{{()()}[()<>]}{([]
[<[({<<<<<<[{{{}{}}{[]<>}}[<{}{}>(<>[])]]({<<>[]><{}()>})><<<{<>()}[()<>]>[<<>[]><<>{}>]>{<{[]<>}[()
([[<<(((<[(({({}{})<[]{}>})<<<{}[]>{()<>}><{[]()}>>)((<{<>()}[<>[]]>[<{}()><{}[]}]))]{<[({(){}}
[({<(<<[{<[[<(<><>)[{}]><([][])[{}<>]>]{([<>[]]({}()))[{[]<>}([]<>)]}]>{<(<[[][]]<[]()>>{(()
{({(([({{{(<({()()}<<>()>)[(<>[])[{}{}]]>[{[()[]]([][]>}[{(){}}<{}<>>]])[(<{[][]}(<>{})>{[[]<>
(<{([(((<{[<{[<><>]{()[]}}<<[]>>><{({}<>)(<>[])}{<{}[]>[[]{})}>]({<{{}<>}(()[])>{([][])[{}
<<(<[({{[<<({{[]()}})<(([][])([]()))>){([(<>{})][<{}<>>{{}[]}])[(({}())(<><>))<<[]><<>[]>>]}>({[{{<>}(<>[])}{
[{[[<<<{(([([({}<>)[<>()]]<(<>())>){([<>][<>()]){{(){}}[<>{}]}}]([<{{}{}}<{}<>>><{<><>}>]<(<<>[]><{}[]>){[
<<{<[([{{[[[{<<>()>({}())}<<(){}>[{}()]>]](([{[]{}}[<>()]][[[]()](()[])]){[[{}()]<[]>][(<><>}]})]}[<
<[(<<(({<{((([<><>]<(){}>)<{{}}({}())>)<[<()<>>[()[]]]({[]}{[]()})>)}><<[(((()<>))<(<><>)>)]([((()<>){{}{}}
{({({<<[(<{{<{(){}}<[][]>>[[<>()]({}<>)]}}><<(<{[]<>}([][])>[<{}<>>([]())]){(<()[]>([]()))}>[({[()<>
<([({{<{([<<[[{}()]][<()[]><<>{}>]>([<{}[]><()[]>]<{{}()}[[]{}]>)>{<(<(){}>)({{}{}}{{}<>})>{([(){}][[][]])[{
<<<<(([<([[{{[{}{}]{<>[]}}[[{}{}]<[]{}>]}[({[]()}[[]<>])[[[]{}]<<>()>]]](<{<[][]}{<>()}}{{[][]}}>{
{[[<[{<{<((<<([]())({}())><(<><>)<{}<>>>)<[{{}[]}][<{}<>>([]{})]>)<{[<{}[]>(<><>)]<<(){}><[]{}>>}{<<<>{}>>
<(<<((({{[({<<{}[]>{<>[]}>{({}{}){{}[]}}}{[[()()](<>{})]{{<><>}<<>[]>}}){<[{()()}<()[]>][[{}{
[<([<<{<<[<<([()[]]({}[]))<{<>()}>>{[<()<>>[{}()]]<{()[]}{(){}>>}>]>><[([([<[]<>>{<>{}}]<([][]){<>{}}>)
[{{([{([(({<{({}[])(()())}{{{}<>}[()()]}>[[[<>[]]([][])]({<><>}<{}{}>)]}[[<{()<>}[<>()]>[<{}{}>[()()]]]{([(
<({[(([[({((<({}[])([]<>)>{<()[]>([]{})}){{([][]){[]{}}}[({}{})[{}]]})<{<{[][]}([]())><[[][]]{<><>}>}<<
[<<((<<[<{{{{<{}[]>[{}[]]}{{<>{}}{<>[]}}}[{(<>[])({}())}{<<>()>(())}]}{<{(()()){()<>}}><[([]())]<<()[]>[(
([[(<<[{[({(<<[]<>><{}>>({<>[]}([]())))[((<>[]){()()}){{[]<>}[[]()]}]}([({[]()}[[]{}])[<{}()>{[]<>}
{(<((<{([<<{{{()[]}<(){}>>[[[]()]<{}<>>]}<([[]<>]){(<>{})<()[]>}>>>{(({[<>()]([]())}))}]([<{({{
[[[[<([{[[{<[{()}(<>[])]>(<[[]{}]<{}{}>>)}]<<((<(){}>{{}[]})[([]{})[[][]]]){{<(){}><[]()>}{(()<>)[[]<>]}
<([{<<[<{{([(<<>()>(<>[]))<[<>[]]>](<({}())<()()>>)>{{{[<>[]]{<>{}}}({(){}}{{}()})}}}({{([()[]][[][]])[
<<({({{<(<(([<[]()>[{}<>]]<({}){()[]}>){[<[]{}>(()<>)]}){[{{<>{}}{()[]}}((<>[]))]{(<{}()>{(){}])[(<>{})<
(<<({(<<([(([<<><>>([][])]<[<>[]][()]>)<(<[]<>>[<>{}])>)(([({}()){[]()}]((()[])(<>()))){<<{}<>>{[]{}}>{[(
([[[{{[<{(({{(()<>){<>{}}}{[{}<>](<>[])}}{{<<><>>{{}[]}}([<><>]{<>()})}))([<([[]<>])>]({[<()[]>][
<<(([([[{({([<<><>><[]{}>][(()())(()[])])<([{}()][{}[]])<((){}){[]()}>>}<{[{[]()}[[]()]]}>)[<((<[]{}>)((<>[])
{<<{[[((<({<{<<>{}>}[(()<>)[{}[]]]>[([<>()]({}()))]}<<<<<>[]><<>>>>{(([]{})[()()])}>)((<((()[])(()<>
<{<{<{<[({(<[(<>[]){<>{}}]{(()[])([]{})}>)}<[({<<>()><{}()>})(([()<>}[[]{}])<{{}<>}>)]>)<(<{{{[]{}}<<>{}>
<[[{(<{(<([<<{{}{}}[[]<>]>><{{(){}}<()<>>}{<{}{}>{[]{}}}>])>{{<{([{}()][[][]])<([][])({}<>)>}>{
({({<<[{[[{{[{{}()}{()[]}]{[{}[]](()())}}<[([][]){()[]}]<[[]()]((){})>>}]<[(<{[]{}>{{}<>}>{
{{[<[((([<[([[()[]]{()())]<({}{}){[]{}}>)]>][<<({<(){}>({}{})}{<<>[]>[()]})<(<(){}>{[]()}){
(({[((<(<({([(()()){<>[]}]){({{}<>}{{}[]})}})[({({{}<>}[{}()]){{[]()}<<>{}>}}<<<()()>[<>{}]>{<{}{}>}>)]>[[
((<[[((<[(({[{<>()}(<>[])]}{[(()())[[][]>][<{}<>>{<><>}]})([{{<><>}({}<>)}]{(<[]<>>(()[])){<{}<
<[[{[<[<{<<<<([]{})[<><>]>(({}())(()<>))><([[]{}][[]()])<{[][]}[<>{}]>>>(({(()[]][[][]]}){{<<><>>{[][]}}}
[<[{{[{[<<{(<([][]){[]<>}><<[][]>([]<>)])<<<[]{}><<>[]>>([{}<>]<[][]>)>}>><{([({<><>}<()()>)<({
{<{([<{(((<{<{[][]}{()()}>(((){}){()()})}>)[({<<<><>){<>()}>((()[]){()[]})}{({<>()}{{}()})({<>{}
[[[(<<{({[{[[<[]<>>[{}()]][[{}[]][{}()]]][[[()<>]([][])]<<()>([]<>)>]}[({[<>]{()<>}}(<<>()><(){}>))[<({}[
<[{({<{[<[(<({{}[]}<<>()>)([{}<>]{[]{}})>)[(<{()}<()>>{[()<>][()[]]})<[[()()][{}<>]])]]<[[{(<>[]){<>()}}
<((<{[{{<{([{<()()>(()[])}{{[]<>}[[]{}]}](((<>())){{[][]}<{}[]>}))(<{((){})[<>{}]}[[<>()]<<>[]>]><{({}<>)({
{<[[<(([<[[[{{[]()}{<>{}}}<<<>[]>>][(({}{}){<>[]})[(<>{}){()<>}]]]<{<((){})([][])>}<[((){})[[]{}
([[{[[<{<((<(<<>>)>){((<{}<>>{<>()})){([{}[]}{<>()})<<{}()><<><>>>}})<{{([(){}])<<{}[]>{[]<>}>}<<<()<>>[[
<[([[[[[{((<{<(){}><<><>>}[{[]<>}[<><>]]>[({()[]}{<>[])){{{}<>}[[]{}]}])<[([[]<>][{}])([[]()][{}()])]<<{[
[{[<[<(<[([[([<>[]]<(){}>){(<><>)<{}<>>}]<[[[]()][<><>]]{{[]<>}(()[])}>]){{<{{[]{}}}((<>()){[][]})
{{([[({{(<<(<[{}[]]<<>[]>><{{}<>}{(){}}>)>>)}})((<{<{<[([][])][[<>{}]<()[]>])}<([[{}[]]{[]()}]{
<({{{{{{<(<<[<()()>([]<>)](<[]()>[<>{}])>(<{<><>}(<>[])>)>[({(<>())<<>{}>})])><<<[{[[]()]<{
{[([(<([[({<({{}[]}{{}()})<<{}<>>{<>{}}>>})([([{[][]){{}<>}]{{<>}[()[]]})((<(){}>[[]()])[[[][]]([]<>)])])]{
{(<[<{[[{({<[{<>[]}<()<>>](<{}()><{}{}>)><<({}<>)<{}()>>([<><>][{}()])>})}<[{<([()[]])>}]>](<({(<(<>())(<><
({<(({{{[[{<([[]<>])>(<{<>}>[[()()][()()]])}]]}<<[<[<<{}[]>((){})>{([]{})}]><<{<{}()>{[]()}}{{()()}<
<{<<(<[<{[<[{<{}{}>[<>{}]}[{[]<>}{<><>}]]{{<()<>>}[<{}()><{}()>]]>]}>]>)>>}<<[<(([[[<((<(){}>){([]{}
<<(<[[<[[[<<({<>{}}{()})><[([][]){[]<>}][[()[]]]>>]]{[{(<(<>{})({}{})>)}]({[({{}<>}([]<>))([()[]
<[[(({[<(<{([(()()){()[]}]<(()[]){<>()}>)<[<(){}><[]{}>]({[][]}<[]{}>)>}[<[(<>)<<>{}>)>({{<>{}}(()
([[({[<<{(<[<{{}{}}<{}{}>>{({}())(()<>)}]>[([{[]}(<>{})])])}>>]{[{[<{<(<<>{}><{}[]>)><({[]<>}<{}<>
{(<(<[<{[<{<({()[]}){({})}>}{[<[{}[]](<>())>{[{}<>](()[])}]}>]{{<[[[<><>]]](<(<>{})><{[]()}>)>}{[<{[{}{}]((
{{{(({({<[[[[[<>{}]<[]{}>][((){}){{}{}}]][[([]<>)<[]{}>]{({}())(<>[])}]][<((()<>)[()])><{[{}[]]{[][]}}{{
{<[({{([({(([<{}[]>{()[]}]){<[[][]][<>{}]>[{<>[]}({})]})}{[{[{()()}{<>()}]}{[([]{})<<><>>]}]})<<
{<<<{{({{<([<({}{})<<>{}>><[[][]]({}<>)>])>([{([{}()])}([<(){}>{{}()}](<<>{}>(<>[])))](<([<
<{{{<({[[{{<<[(){}]<[]<>>>{{{}()}([]{})}>}<<[<<>{}><{}[]>]([(){}][[]{}])><[{{}{}}<{}<>>][<[][]>
((<(<<{<(<{[[{[][]}]{{()<>}<<>{}>}](<[[]<>]<{}()>>((()())[<>{}]))}[([(<>{})<()[]>]<[[][]]<[]{}
{[<{[[{<[((<<<<>[]>[[]{}]>({{}[]>)>)<{[[[]{}]<{}{}>]<([][])(()<>)>}>)](<[[<[<>()]({}[])>]]>({{<<[]{}>{[]
<[{{[(<<([[[[<<>()><{}<>>](((){}))]([<{}{}>[<>()]](<[]()>[[]()]))][{{{<>}{[]{}}}{{{}<>}({}<>)}}[{({}{}>{{}<>}
<[<{{([{[{[((([]())({}<>)){[[]()][{}{}]})]({{(()[])[()()]}})>(({[({}())[<>()]]{<<>{}>[<>{}]}})[[{[()<>]<<>
[{([<<[<[{{<<{<>[]}({}[])>>{<{{}{}}[<><>]>([[]<>]<{}[]>)}}{[([()[]](<>))<{{}[]}<(){}>]]}}[[<{[<>[]]{[]<
<[[([<{<{<[([[()[]]])<({[]()}({}()))[(<>{}){<>[]}]>]>(([<{{}()}(()[])>{{{}<>><<>[]>}][[({}<>)[<>[]]]{{
[<((({((<<[[[{{}<>}{()<>}]]({(<>[])<()<>>}{(()()){{}{}}})]<<(({}<>))>{<<[]()>><[()()]<[]{}>>}>>>)(<<{<{<{
(<(({(<(<<([<([]<>)[{}{}>>({{}()}[{}[]])]<({[]<>}[{}{}])<([][]){{}{}}>>)>{[(<<[][]><{}()>>{{<>{"#;

        let ans = day_10_part_1(&input);

        assert_eq!(ans, 387363);
    }
}