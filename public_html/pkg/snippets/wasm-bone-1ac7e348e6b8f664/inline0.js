export async function FasBoneReady(){var a=[],e=(a.uri=window.location.href.split(window.location.hostname)[1],a.query_string=location.search.substring(1),navigator.userAgent.toLowerCase()),e=(a.isMobile=!(-1!==e.indexOf("windows nt")||-1!==e.indexOf("mac os x")),new Date),e=(a.Year=e.getFullYear(),a.Month=e.getMonth()+1,a.Week=e.getDay(),a.Day=e.getDate(),a.hour=e.getHours(),a.min=e.getMinutes(),a.sec=e.getSeconds(),a.YYYYMMDD=a.Year+"年"+a.Month+"月"+a.Day+"日",a.protpcol=void 0,a.token=void 0,a.apihost=void 0,a.event_string=void 0,window.clientinfo=a,await fetch("/json/static/route.json")),e=e.ok?await e.json():JSON.parse('{"result":"'+e.status+" "+e.message+'."}');if(void 0===e[a.uri]){let t=404;Object.keys(e).forEach(function(e){1==a.uri.startsWith(e)&&"/"!=a.uri&&(a.event_string=a.uri.replace(e,""),t=e)}),a.uri=t}var t=await fetch("/template/action/"+e[a.uri].action_name+".template"),t=t.ok?await t.text():"読込エラー",i=e[a.uri].api_urlandpath,i=(i.indexOf("json/api")<0&&(wasm.gethost(),void 0===a.token)&&(i=i.replace("api","static")),await fetch(i)),i=(a.page_api=i.ok?await i.json():JSON.parse('{"result":"'+i.status+" "+i.message+'."}'),_.template(t)),t=(document.getElementsByTagName("body")[0].innerHTML=i(a.page_api),document.createElement("script")),i=(document.head.appendChild(t),t.type="text/javascript",t.id="action_execute",t.src="/js/action/"+e[a.uri].action_name+".js",t.onload=function(e){Execute()},document.createElement("script"));document.head.appendChild(i),i.type="text/javascript",i.id="com_environment",i.src="/js/common/com_envrironment.js"}