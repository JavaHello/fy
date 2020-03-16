use std::collections::HashMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::Deserialize;

const APP_KEY: &'static str = "";
const APP_SECRET: &'static str = "";
const YOUDAO_URL: &'static str = "https://openapi.youdao.com/api?";

fn init_error_map() -> HashMap<&'static str, &'static str> {
    let mut map: HashMap<&'static str, &'static str> = HashMap::new();
    map.insert("101", "缺少必填的参数");
    map.insert("102", "不支持的语言类型");
    map.insert("103", "翻译文本过长");
    map.insert("104", "不支持的API类型");
    map.insert("105", "不支持的签名类型");
    map.insert("106", "不支持的响应类型");
    map.insert("107", "不支持的传输加密类型");
    map.insert("108", "应用ID无效，注册账号，登录后台创建应用和实例并完成绑定，可获得应用ID和应用密钥等信息");
    map.insert("109", "batchLog格式不正确");
    map.insert("110", "无相关服务的有效实例");
    map.insert("111", "开发者账号无效");
    map.insert("112", "请求服务无效");
    map.insert("113", "q不能为空");
    map.insert("114", "不支持的图片传输方式");
    map.insert("201", "解密失败，可能为DES,BASE64,URLDecode的错误");
    map.insert("202", "签名检验失败");
    map.insert("203", "访问IP地址不在可访问IP列表");
    map.insert("205", "请求的接口与应用的平台类型不一致，如有疑问请参考入门指南");
    map.insert("206", "因为时间戳无效导致签名校验失败");
    map.insert("207", "重放请求");
    map.insert("301", "辞典查询失败");
    map.insert("302", "翻译查询失败");
    map.insert("303", "服务端的其它异常");
    map.insert("304", "会话闲置太久超时");
    map.insert("401", "账户已经欠费停");
    map.insert("402", "offlinesdk不可用");
    map.insert("411", "访问频率受限,请稍后访问");
    map.insert("412", "长请求过于频繁，请稍后访问");
    map.insert("1001", "无效的OCR类型");
    map.insert("1002", "不支持的OCR image类型");
    map.insert("1003", "不支持的OCR Language类型");
    map.insert("1004", "识别图片过大");
    map.insert("1201", "图片base64解密失败");
    map.insert("1301", "OCR段落识别失败");
    map.insert("1411", "访问频率受限");
    map.insert("1412", "超过最大识别字节数");
    map.insert("2003", "不支持的语言识别Language类型");
    map.insert("2004", "合成字符过长");
    map.insert("2005", "不支持的音频文件类型");
    map.insert("2006", "不支持的发音类型");
    map.insert("2201", "解密失败");
    map.insert("2301", "服务的异常");
    map.insert("2411", "访问频率受限,请稍后访问");
    map.insert("2412", "超过最大请求字符数");
    map.insert("3001", "不支持的语音格式");
    map.insert("3002", "不支持的语音采样率");
    map.insert("3003", "不支持的语音声道");
    map.insert("3004", "不支持的语音上传类型");
    map.insert("3005", "不支持的语言类型");
    map.insert("3006", "不支持的识别类型");
    map.insert("3007", "识别音频文件过大");
    map.insert("3008", "识别音频时长过长");
    map.insert("3009", "不支持的音频文件类型");
    map.insert("3010", "不支持的发音类型");
    map.insert("3201", "解密失败");
    map.insert("3301", "语音识别失败");
    map.insert("3302", "语音翻译失败");
    map.insert("3303", "服务的异常");
    map.insert("3411", "访问频率受限,请稍后访问");
    map.insert("3412", "超过最大请求字符数");
    map.insert("4001", "不支持的语音识别格式");
    map.insert("4002", "不支持的语音识别采样率");
    map.insert("4003", "不支持的语音识别声道");
    map.insert("4004", "不支持的语音上传类型");
    map.insert("4005", "不支持的语言类型");
    map.insert("4006", "识别音频文件过大");
    map.insert("4007", "识别音频时长过长");
    map.insert("4201", "解密失败");
    map.insert("4301", "语音识别失败");
    map.insert("4303", "服务的异常");
    map.insert("4411", "访问频率受限,请稍后访问");
    map.insert("4412", "超过最大请求时长");
    map.insert("5001", "无效的OCR类型");
    map.insert("5002", "不支持的OCR image类型");
    map.insert("5003", "不支持的语言类型");
    map.insert("5004", "识别图片过大");
    map.insert("5005", "不支持的图片类型");
    map.insert("5006", "文件为空");
    map.insert("5201", "解密错误，图片base64解密失败");
    map.insert("5301", "OCR段落识别失败");
    map.insert("5411", "访问频率受限");
    map.insert("5412", "超过最大识别流量");
    map.insert("9001", "不支持的语音格式");
    map.insert("9002", "不支持的语音采样率");
    map.insert("9003", "不支持的语音声道");
    map.insert("9004", "不支持的语音上传类型");
    map.insert("9005", "不支持的语音识别 Language类型");
    map.insert("9301", "ASR识别失败");
    map.insert("9303", "服务器内部错误");
    map.insert("9411", "访问频率受限（超过最大调用次数）");
    map.insert("9412", "超过最大处理语音长度");
    map.insert("10001", "无效的OCR类型");
    map.insert("10002", "不支持的OCR image类型");
    map.insert("10004", "识别图片过大");
    map.insert("10201", "图片base64解密失败");
    map.insert("10301", "OCR段落识别失败");
    map.insert("10411", "访问频率受限");
    map.insert("10412", "超过最大识别流量");
    map.insert("11001", "不支持的语音识别格式");
    map.insert("11002", "不支持的语音识别采样率");
    map.insert("11003", "不支持的语音识别声道");
    map.insert("11004", "不支持的语音上传类型");
    map.insert("11005", "不支持的语言类型");
    map.insert("11006", "识别音频文件过大");
    map.insert("11007", "识别音频时长过长，最大支持30s");
    map.insert("11201", "解密失败");
    map.insert("11301", "语音识别失败");
    map.insert("11303", "服务的异常");
    map.insert("11411", "访问频率受限,请稍后访问");
    map.insert("11412", "超过最大请求时长");
    map.insert("12001", "图片尺寸过大");
    map.insert("12002", "图片base64解密失败");
    map.insert("12003", "引擎服务器返回错误");
    map.insert("12004", "图片为空");
    map.insert("12005", "不支持的识别图片类型");
    map.insert("12006", "图片无匹配结果");
    map.insert("13001", "不支持的角度类型");
    map.insert("13002", "不支持的文件类型");
    map.insert("13003", "表格识别图片过大");
    map.insert("13004", "文件为空");
    map.insert("13301", "表格识别失败");
    map.insert("15001", "需要图片");
    map.insert("15002", "图片过大（1M）");
    map.insert("15003", "服务调用失败");
    map.insert("17001", "需要图片");
    map.insert("17002", "图片过大（1M）");
    map.insert("17003", "识别类型未找到");
    map.insert("17004", "不支持的识别类型");
    map.insert("17005", "服务调用失败");
    map
}

#[derive(Deserialize)]
#[derive(Debug)]
struct FyResp {
    errorCode: String,
    query: Option<String>,
    translation: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let err_map = init_error_map();
    let mut arg_list = env::args();
    &arg_list.next();
    let mut q = String::new();
    for it in arg_list {
        q.push_str(&it);
        q.push(' ');
    }
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("q".to_owned(), q.to_owned());
    let mut qry = gen_qry(&mut data);
    qry.insert_str(0, YOUDAO_URL);
    // println!("{}", qry);
    let resp: FyResp = match reqwest::blocking::get(qry.as_str())?
        .json() {
        Ok(resp) => resp,
        Err(_) => FyResp { errorCode: String::from("17005"), query: Some(q.to_owned()), translation: None },
    };
    let err_code = resp.errorCode.as_str();
    if err_map.contains_key(err_code) {
        println!("{}", err_map.get(err_code).unwrap());
    } else {
        println!("原文: {}", q);
        print!("翻译: ");
        for item in resp.translation.unwrap() {
            println!("{}", item);
        }
    }
    Ok(())
}

/**
 *
 *
 * 字段名	类型	含义	必填	备注
 * q	text	待翻译文本	True	必须是UTF-8编码
 * from	text	源语言	True	参考下方 支持语言 (可设置为auto)
 * to	text	目标语言	True	参考下方 支持语言 (可设置为auto)
 * appKey	text	应用ID	True	可在 应用管理 查看
 * salt	text	UUID	True	UUID
 * sign	text	签名	True	sha256(应用ID+input+salt+curtime+应用密钥)
 * signType	text	签名类型	True	v3
 * curtime	text	当前UTC时间戳(秒)	true	TimeStamp
 * ext	text	翻译结果音频格式，支持mp3	false	mp3
 * voice	text	翻译结果发音选择	false	0为女声，1为男声。默认为女声
 * strict	text	是否严格按照指定from和to进行翻译：true/false	false	如果为false，则会自动中译英，英译中。默认为false
 *
 * 签名生成方法如下：
 * signType=v3；
 * sign=sha256(应用ID+input+salt+curtime+应用密钥)；
 * 其中，input的计算方式为：input=q前10个字符 + q长度 + q后10个字符（当q长度大于20）或 input=q字符串（当q长度小于等于20）；
*/

fn gen_qry(data: &mut HashMap<String, String>) -> String {
    let now = SystemTime::now();
    data.insert("from".to_owned(), "auto".to_owned());
    data.insert("to".to_owned(), "auto".to_owned());
    data.insert("appKey".to_owned(), APP_KEY.to_owned());
    data.insert("salt".to_owned(), now.duration_since(UNIX_EPOCH).unwrap().as_nanos().to_string());
    data.insert("signType".to_owned(), "v3".to_owned());
    data.insert("curtime".to_owned(), now.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());
    let mut m = Sha256::new();
    m.input_str(sign_str(data).as_str());
    data.insert("sign".to_owned(), m.result_str());
    return query_str(data);
}

fn sign_str(data: &mut HashMap<String, String>) -> String {
    let mut result = String::new();
    result.push_str(data.get("appKey").unwrap());
    let sign_str = input(data.get("q").unwrap());
    result.push_str(sign_str.as_str());
    result.push_str(data.get("salt").unwrap());
    result.push_str(data.get("curtime").unwrap());
    result.push_str(APP_SECRET);
    return result;
}

fn input(q: &str) -> String {
    let chs = q.chars();
    let l = chs.count();
    if l <= 20 {
        q.to_owned()
    } else {
        // chs[(l - 10)..l];
        let v: Vec<char> = q.chars().collect();
        let mut result = String::new();
        for x in 0..10 {
            result.push(v[x]);
        }
        result += &l.to_string();
        let mut i = l - 10;
        while i < l {
            result.push(v[i]);
            i += 1;
        }
        result
    }
}

fn query_str(data: &mut HashMap<String, String>) -> String {
    let mut result = String::new();
    for item in data.iter() {
        result.push_str(item.0);
        result.push_str("=");
        result.push_str(item.1);
        result.push_str("&");
    }
    return result;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{sign_str, APP_SECRET} ;

    #[test]
    fn test_sign_str() {
        let mut map:HashMap<String, String> = HashMap::new();
        map.insert("appKey".to_owned(), "a".to_owned());
        map.insert("q".to_owned(), "b".to_owned());
        map.insert("salt".to_owned(), "c".to_owned());
        map.insert("curtime".to_owned(), "d".to_owned());
        let res = sign_str(&mut map);
        let mut t = String::from("abcd");
        t.push_str(APP_SECRET);
        assert_eq!(t, res);

        map.insert("q".to_owned(), "123456789axxx一二三四五六七八九十".to_owned());

        let res = sign_str(&mut map);
        let mut t = String::from("a123456789a23一二三四五六七八九十cd");
        t.push_str(APP_SECRET);
        assert_eq!(t, res);
    }
}
