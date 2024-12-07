/*

どんどん共通関数を足していこう
関数内は厳格モード実施！！
'use strict';
*/


alert("com_envrironment.js Loading Complete."); 
//-----------------------------------------------------//
// 画面の発火点【真のシーケンス】
//-----------------------------------------------------//



    //FasBoneReady();
    //async function FasBoneReady() {'use strict';var clientinfo = [];clientinfo["uri"]=window.location.href.split(window.location.hostname)[1];clientinfo["query_string"]=location.search.substring(1);var ua = navigator.userAgent.toLowerCase();clientinfo["isMobile"] = !(ua.indexOf("windows nt") !== -1 || ua.indexOf("mac os x") !== -1);var cur_date=new Date(); clientinfo["Year"] = cur_date.getFullYear();clientinfo["Month"] = cur_date.getMonth()+1;clientinfo["Week"] = cur_date.getDay();clientinfo["Day"] = cur_date.getDate();clientinfo["YYYYMMDD"] = clientinfo["Year"] + '年' + clientinfo["Month"] + '月' + clientinfo["Day"] + '日';const response1 = await fetch("/json/static/route.json");var route_json = (!response1.ok) ? JSON.parse('{"result":"' +  response1.status + ' ' +  response1.message +  '."}') : await response1.json();const response2 = await fetch('/template/action/' + route_json[clientinfo["uri"]]['action_name'] + '.template');var Template = (!response2.ok) ? '読込エラー' : await response2.text();const response3 = await fetch(route_json[clientinfo["uri"]]['api_urlandpath']);var page_api = (!response3.ok) ? JSON.parse('{"result":"' +  response3.status + ' ' +  response3.message +  '."}') : await response3.json();var bodyCompiled = _.template(Template);document.getElementsByTagName('body')[0].innerHTML=bodyCompiled(page_api);var action_execute = document.createElement('script');document.head.appendChild(action_execute);action_execute.type='text/javascript';action_execute.id = 'action_execute';action_execute.src = '/js/action/'+ route_json[clientinfo["uri"]]['action_name'] + '.js';action_execute.onload = function(e){Execute(clientinfo);};};



/*
async function FasBoneReady() {
        'use strict';
        var clientinfo = [];
        //-----------------------------------------------------//
        // URIの取得 
        //-----------------------------------------------------//
        clientinfo["uri"]=window.location.href.split(window.location.hostname)[1];
    
        //-----------------------------------------------------//
        // QUERY_STRINGSの取得
        //-----------------------------------------------------//
        clientinfo["query_string"]=location.search.substring(1);
    
        //-----------------------------------------------------//
        // 端末の取得 PCはfalse MBはtrue
        //-----------------------------------------------------//
        var ua = navigator.userAgent.toLowerCase();
        clientinfo["isMobile"] = !(ua.indexOf("windows nt") !== -1 || ua.indexOf("mac os x") !== -1);
    
        //-----------------------------------------------------//
        // 本日の年月日を取得
        //-----------------------------------------------------//
        var cur_date=new Date(); 
        clientinfo["Year"] = cur_date.getFullYear();
        clientinfo["Month"] = cur_date.getMonth()+1;
        clientinfo["Week"] = cur_date.getDay();
        clientinfo["Day"] = cur_date.getDate();
clientinfo["hour"] = cur_date.getHours();clientinfo["min"] = cur_date.getMinutes();clientinfo["sec"] = cur_date.getSeconds();



        clientinfo["YYYYMMDD"] = clientinfo["Year"] + '年' + clientinfo["Month"] + '月' + clientinfo["Day"] + '日';
        //リソースを取得
        const response1 = await fetch("/json/static/route.json");
        var route_json = (!response1.ok) ? JSON.parse('{"result":"' +  response1.status + ' ' +  response1.message +  '."}') : await response1.json();
        //-----------------------------------------------------//
        // 真っ先にCSSファイル
        //-----------------------------------------------------//
        var page_css=document.createElement("link");
        page_css.setAttribute("rel","stylesheet");
        page_css.setAttribute("type”,”text/css");
        page_css.setAttribute("href",'/css/action/'+ route_json[clientinfo["uri"]]['action_name'] + '.css');
        document.getElementsByTagName("head")[0].appendChild(page_css);

        //-----------------------------------------------------//
        // アクションファイルとテンプレートファイル
        //-----------------------------------------------------//
        //var Action=route_json[v_uri]['action_name'];//アクション別のjs各フォルダ内のJSファイル
        //var ApiUrlAndPath=route_json[v_uri]['api_urlandpath'];//アクション別のjs各フォルダ内のJSファイル 
        //var Execute='/js/action/'+ route_json[clientinfo["uri"]]['action_name'] + '.js';//今回実行するアクション個別のロジックファイル
        const response2 = await fetch('/template/action/' + route_json[clientinfo["uri"]]['action_name'] + '.template');
        var Template = (!response2.ok) ? '読込エラー' : await response2.text();
var api_urlandpath = route_json[clientinfo["uri"]]['api_urlandpath'];if (api_urlandpath.indexOf('json/api') < 0) {wasm.gethost();if (clientinfo["token"] === undefined) {api_urlandpath=api_urlandpath.replace('api','static');}}const response3 = await fetch(api_urlandpath);




        const response3 = await fetch(api_urlandpath);
        var page_api = (!response3.ok) ? JSON.parse('{"result":"' +  response3.status + ' ' +  response3.message +  '."}') : await response3.json();
        var bodyCompiled = _.template(Template);
        document.getElementsByTagName('body')[0].innerHTML=bodyCompiled(page_api);
        var action_execute = document.createElement('script');
        document.head.appendChild(action_execute);
        action_execute.type='text/javascript';
        action_execute.id = 'action_execute';
        action_execute.src = '/js/action/'+ route_json[clientinfo["uri"]]['action_name'] + '.js';
        action_execute.onload = function(e){
            // 実行したい処理を書く
            Execute(clientinfo);
        };
};
*/

//テンプレートをセットする
async function settemplate(templat_name) {
    const response = await fetch('/template/common/' + templat_name + '.template');
    var Template = (!response2.ok) ? '読込エラー' : await response.text();
    return _.template(Template);
}


//async function GetHost() {    clientinfo["protpcol"] = window.location.href.split(':')[0] + '://';    const response4 = await fetch('/js/apihost');    clientinfo["apihost"] = (!response4.ok) ? undefined : await response4.text();    var apihost = clientinfo["protpcol"] + clientinfo["apihost"] + '/json/api/';    const response5 = await fetch(apihost, {        signal: AbortSignal.timeout(500)    });    var response5_perse = (!response5.ok) ? undefined : await response5.text();    clientinfo["token"] = (response5_perse === undefined) ? undefined : JSON.parse(response5_perse).token;　　clientinfo["is_private"] = (response5_perse === undefined) ? undefined : JSON.parse(response5_perse).is_private;    clientinfo["json_path"] = (response5_perse === undefined) ? '/json/static/' : apihost;    if (response5.ok) {        var inputs = document.getElementsByTagName('input');        for (var i = 0; i < inputs.length; i++) {            if (inputs[i].getAttribute('type') == 'submit') {                inputs[i].disabled = false;            }        }        for (var i = 0; i < document.getElementsByName('_token').length; i++) {            document.getElementsByName('_token')[i].value = clientinfo["token"];        }    }}
//export async function FasBoneReady() {    {        'use strict';        var clientinfo = [];        clientinfo["uri"] = window.location.href.split(window.location.hostname)[1];        clientinfo["query_string"] = location.search.substring(1);        var ua = navigator.userAgent.toLowerCase();        clientinfo["isMobile"] = !(ua.indexOf("windows nt") !== -1 || ua.indexOf("mac os x") !== -1);        var cur_date = new Date();        clientinfo["Year"] = cur_date.getFullYear();        clientinfo["Month"] = cur_date.getMonth() + 1;        clientinfo["Week"] = cur_date.getDay();        clientinfo["Day"] = cur_date.getDate();        clientinfo["hour"] = cur_date.getHours();        clientinfo["min"] = cur_date.getMinutes();        clientinfo["sec"] = cur_date.getSeconds();        clientinfo["YYYYMMDD"] = clientinfo["Year"] + '年' + clientinfo["Month"] + '月' + clientinfo["Day"] + '日';        clientinfo["protpcol"] = undefined;        clientinfo["token"] = undefined;        clientinfo["apihost"] = undefined;        clientinfo["event_string"] = undefined;        window.clientinfo = clientinfo;        const response1 = await fetch("/json/static/route.json");        var route_json = (!response1.ok) ? JSON.parse('{"result":"' + response1.status + ' ' + response1.message + '."}') : await response1.json();        if (route_json[clientinfo["uri"]] === undefined){            let s_uri = 404;            Object.keys(route_json).forEach(function(key) {                if (clientinfo["uri"].startsWith(key) == true && key !="/"){                    clientinfo["event_string"] =clientinfo["uri"].replace(key,'');                    s_uri = key;                };            });            clientinfo["uri"] = s_uri;        };        const response2 = await fetch('/template/action/' + route_json[clientinfo["uri"]]['action_name'] + '.template');        var Template = (!response2.ok) ? '読込エラー' : await response2.text();        var api_urlandpath = route_json[clientinfo["uri"]]['api_urlandpath'];        if (api_urlandpath.indexOf('json/api') < 0) {            wasm.gethost();            if (clientinfo["token"] === undefined) {                api_urlandpath = api_urlandpath.replace('api', 'static');            }        }        ;const response3 = await fetch(api_urlandpath);        clientinfo["page_api"] = (!response3.ok) ? JSON.parse('{"result":"' + response3.status + ' ' + response3.message + '."}') : await response3.json();        var bodyCompiled = _.template(Template);        document.getElementsByTagName('body')[0].innerHTML = bodyCompiled(clientinfo["page_api"]);        var action_execute = document.createElement('script');        document.head.appendChild(action_execute);        action_execute.type = 'text/javascript';        action_execute.id = 'action_execute';        action_execute.src = '/js/action/' + route_json[clientinfo["uri"]]['action_name'] + '.js';        action_execute.onload = function(e) {            Execute();        }        ;        var com_environment = document.createElement('script');        document.head.appendChild(com_environment);        com_environment.type = 'text/javascript';        com_environment.id = 'com_environment';        com_environment.src = '/js/common/com_envrironment.js';    }}
//async function BoneRender(templat_name,tag_id,json_line='{}') {const response = await fetch('/template/common/' + templat_name + '.template');const TemplateHTML = (!response.ok) ? '読込エラー' : await response.text();const Template =_.template(TemplateHTML);document.getElementById(tag_id).innerHTML=Template(JSON.parse(json_line=="" ? null: json_line));}
//async function FasconExtendForm(form_name,validation_only){    const req=JSON.parse(ReqDataForm(form_name,validation_only));   return await FasconExtendManual(req.request_method,req.request_url,reqserialize)};
//async function FasconExtendForm(form_name,validation_only){    const req=JSON.parse(ReqDataForm(form_name,validation_only));   return await FasconExtendManual(req.request_method,req.request_url,reqserialize)};
function ReqDataForm(form_name,validation_only){
    let returnarray= new Object();
    let form_select = document.querySelector('[name="'+ form_name +'"]');
    let form_data = new FormData(form_select);
    validation_only = (validation_only===undefined) ? false : validation_only;
    const formData = new URLSearchParams();
    for (let value of form_data.entries()) {
        formData.append(value[0], value[1]);
    };
    returnarray.serialize=formData.toString();
    returnarray.request_method=form_select.method.toUpperCase();
    returnarray.request_url = (validation_only == true && returnarray.request_method =='POST') ? form_select.action + '?validation_only': form_select.action ;
    return JSON.stringify(returnarray);
};

async function FasconExtendManual(request_method,request_url,serialize){
    const response = await fetch(request_url , {
        method: request_method,
        headers: {'Content-Type': 'application/x-www-form-urlencoded'},
        body: serialize
    });
    var result=await response.text();
    if (!response.ok && !result) 
        result='{"result":"' + response.status + ' ' + response.statusText + '."}';
    return result;
};

async function FasconExtendForm(form_name,validation_only){
    const req=JSON.parse(ReqDataForm(form_name,validation_only));
    return await FasconExtendManual(request_method,request_url,serialize)
};








clientinfo["uri"].startsWith(key) == true && clientinfo["uri"] !="/"

Object.keys(route_json).forEach(function(key) {console.log();});