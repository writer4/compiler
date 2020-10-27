#[test]
#[cfg(feature = "html-backend")]
fn html() {
    let source_code = r###"
# Hello ~~World~~!
## Subtitle
### ...
#### ...
##### ...
###### ...

Lorem ipsum **dolor** sit amet, consetetur sadipscing elitr,
sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat,
sed diam __voluptua__.

Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.

Red **Green
Blue** Yellow

- Red
- **__~~Green~~__**, ~~White
Lorem~~ ipsum __dolor sit amet,
consetetur sadipscing__ elitr, ...
- Blue

- A
- B
- C
    - D
    - E
    -
    - F
    -
    G, H, I, J
K, L, M
    - N
        - O
            - P
            -Q
    - R
- 
- S, T, U, V, W
    - X, Y, Z

~~...__
####### ...
###Hello World
  - A
- B
  - C
 - D
...
  - E
   - F
     -G
- 
 - H
- I
  - J
- K

    - L
    - M
- N
    - O
    - P

### <i>Hello</i>
<b>bold</b>
<s>...<
>.42


"###;

    let expected_output = r###"<div class="writer-doc"><h1>Hello <s>World</s>!</h1><h2>Subtitle</h2><h3>...</h3><h4>...</h4><h5>...</h5><h6>...</h6><p>Lorem ipsum <b>dolor</b> sit amet, consetetur sadipscing elitr,<br />sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat,<br />sed diam <i>voluptua</i>.</p><p>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.</p><p>Red <b>Green<br />Blue</b> Yellow</p><ul><li>Red</li><li><b><i><s>Green</s></i></b>, <s>White<br />Lorem</s> ipsum <i>dolor sit amet,<br />consetetur sadipscing</i> elitr, ...</li><li>Blue</li></ul><ul><li>A</li><li>B</li><li>C<ul><li>D</li><li>E</li><li></li><li>F</li><li><br />    G, H, I, J<br />K, L, M</li><li>N<ul><li>O<ul><li>P<br />            -Q</li></ul></li></ul></li><li>R</li></ul></li><li></li><li>S, T, U, V, W<ul><li>X, Y, Z</li></ul></li></ul><p>~~...__<br />####### ...<br />###Hello World</p><ul><li>A</li><li>B<ul><li>C</li></ul></li><li>D<br />...</li><li>E</li><li>F<br />     -G</li><li></li><li>H</li><li>I<ul><li>J</li></ul></li><li>K</li></ul><ul><li>L</li><li>M</li><li>N<ul><li>O</li><li>P</li></ul></li></ul><h3>&lt;i&gt;Hello&lt;/i&gt;</h3><p>&lt;b&gt;bold&lt;/b&gt;<br />&lt;s&gt;...&lt;<br />&gt;.42</p></div>"###;

    assert_eq!(
        compiler::compile_html(source_code).unwrap(),
        expected_output
    );
}
