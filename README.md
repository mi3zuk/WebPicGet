# WebPicGet
**Created by [Copiot](https://copilot.microsoft.com/)**

<img src="WebPicGet.png" width="10%">

## from URL モード
URLを入力して、一括でダウンロード<br>

### cookie
cookie情報が必要な場合に入力する<br>
「変数名=値」のフォーマットで入力<br>
セミコロン（;）で区切れば複数指定可能<br>

### Num. sequence
同じページ内で画像ファイル名が番号順になっている場合に使用<br>

digit: 桁数（「pic001.jpeg」などの名前のときに使用）<br>
init: 初期値<br>
end: 終了値<br>
gain: 増加量（1, 3, 5, 7など、値を飛ばしたいときに使用）<br>

値を入力し終わったら、「Add」ボタンでダウンロードリストに追加される<br>

### Download list
左の×で削除、改行で行追加<br>
List Clean: リスト内のURLを全て消す<br>
Output Folder: ダウンロード先のフォルダを指定する<br>
Get: ダウンロードを開始する<br>
Add from Log: ログのダウンロードが失敗したURLを再度ダウンロードリストに追加する<br>

### Observe Clipboard
クリップボードが更新される度に、その内容をダウンロードリストに追加する<br>
ペーストする手間が省ける<br>

## search
入力したURLからimgタグを入手<br>
精度はあんまり良くない<br>
