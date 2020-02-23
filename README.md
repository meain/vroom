# [WIP] Vroom

Run VIM macros over multiple lines.

### Usage

```
vroom 'pattern' filename
```
 *Use `-` for filename to read from stdin*

<table>
<tr>
<th><code>cat list</code></th>
<th><code>vroom 'A juice<esc>I- ' list</code></th>
<th><code>vroom '$a pie,' list</code></th>
<th><code>vroom '0rW' list</code></th>
</tr>
<tr>
<td><pre>
lemon
mango
tomato
orange
apple
</pre></td>
<td><pre>
- lemon juice
- mango juice
- tomato juice
- orange juice
- apple juice
</pre></td>
<td><pre>
lemon pie,
mango pie,
tomato pie,
orange pie,
apple pie,
</pre></td>
<td><pre>
Wemon
Wango
Womato
Wrange
Wpple
</pre></td>
</tr>
</table>
