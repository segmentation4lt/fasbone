//-----------------------------------------------------//
//必須で指定 ※ファイル単位?
//-----------------------------------------------------//
'use strict';

//-----------------------------------------------------//
// Actionに応じたAPIリクエストを実施し、結果をDataオブジェクトに反映させる
//-----------------------------------------------------//
function Execute(){
    //-----------------------------------------------------//
    // 動作確認
    //-----------------------------------------------------//
    console.log(clientinfo);//クライアント環境情報

    //-----------------------------------------------------//
    // Render(template出力)処理
    // 引数:<common|action/テンプレート名>,<出力対象タグのid>,<JSON文字列>
    //-----------------------------------------------------//
    //wasm.bonerender("action/template_name","main",clientinfo.page_api.api_data);
    
    //描画終了を確認後、待受処理を実施(発火点)
    const observer = new MutationObserver((mutationsList, observer) => {
        for (let mutation of mutationsList) {
            if (mutation.type === "childList") {
                const targetElement = document.getElementById('main');
                if (targetElement) {//描画終了確認
                    observer.disconnect(); // 監視を停止
                    // イベントを追加
                    EventAction();
                }
            }
        }
    });
    observer.observe(document.body, { childList: true, subtree: true });

};

//-----------------------------------------------------//
// 待ち受け処理をここに記載
//-----------------------------------------------------//
function EventAction(){

};

