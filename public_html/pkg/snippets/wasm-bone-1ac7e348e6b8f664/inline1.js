export async function GetHost(){clientinfo.protocol=window.location.href.split(":")[0]+"://";var t=await fetch("/js/apihost"),t=(clientinfo.apihost=t.ok?await t.text():void 0,clientinfo.protocol+clientinfo.apihost+"/json/api/"),i=await fetch(t,{signal:AbortSignal.timeout(500)}),o=i.ok?await i.text():void 0;if(clientinfo.token=void 0===o?void 0:JSON.parse(o).token,clientinfo.is_private=void 0===o?void 0:JSON.parse(o).is_private,clientinfo.json_path=void 0===o?"/json/static/":t,i.ok){for(var e=document.getElementsByTagName("input"),n=0;n<e.length;n++)"submit"==e[n].getAttribute("type")&&(e[n].disabled=!1);for(n=0;n<document.getElementsByName("_token").length;n++)document.getElementsByName("_token")[n].value=clientinfo.token}}