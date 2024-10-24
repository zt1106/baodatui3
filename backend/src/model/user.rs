use crate::model::configs::GameConfigurations;
use baodatui_macro::ID;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, ID, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub nick_name: String,
    pub uuid: String,
    pub login_timestamp: u64,
    pub prepared: bool,
    pub preferred_game_config: Option<GameConfigurations>,
}

impl Default for User {
    fn default() -> Self {
        let nick_name = create_random_chinese_name();
        let uuid = Uuid::new_v4().to_string();
        Self {
            id: 0,
            nick_name,
            uuid,
            login_timestamp: 0,
            preferred_game_config: None,
            prepared: false,
        }
    }
}

#[test]
fn test_user() {
    let user = User::default();
    println!("{:?}", user);
}

#[test]
fn test_create_random_chinese_name() {
    for _ in 0..10 {
        println!("{}", create_random_chinese_name());
    }
}

pub fn create_random_chinese_name() -> String {
    let surnames = [
        "赵", "钱", "孙", "李", "周", "吴", "郑", "王", "冯", "陈", "褚", "卫", "蒋", "沈", "韩",
        "杨", "朱", "秦", "尤", "许", "何", "吕", "施", "张", "孔", "曹", "严", "华", "金", "魏",
        "陶", "姜", "戚", "谢", "邹", "喻", "柏", "水", "窦", "章", "云", "苏", "潘", "葛", "奚",
        "范", "彭", "郎", "鲁", "韦", "昌", "马", "苗", "凤", "花", "方", "俞", "任", "袁", "柳",
        "酆", "鲍", "史", "唐", "费", "廉", "岑", "薛", "雷", "贺", "倪", "汤", "滕", "殷", "罗",
        "毕", "郝", "邬", "安", "常", "乐", "于", "时", "傅", "皮", "卞", "齐", "康", "伍", "余",
        "元", "卜", "顾", "孟", "平", "黄", "和", "穆", "萧", "尹", "姚", "邵", "湛", "汪", "祁",
        "毛", "禹", "狄", "米", "贝", "明", "臧", "计", "伏", "成", "戴", "谈", "宋", "茅", "庞",
        "熊", "纪", "舒", "屈", "项", "祝", "董", "梁", "杜", "阮", "蓝", "闵", "席", "季",
    ];
    let mut result = surnames[rand::thread_rng().gen_range(0..134)].to_string();
    let female_names = "秀娟英华慧巧美娜静淑惠珠翠雅芝玉萍红娥玲芬芳燕彩春菊兰凤洁梅琳素云莲真环雪荣爱妹霞香月莺媛艳瑞凡佳嘉琼勤珍贞莉桂娣叶璧璐娅琦晶妍茜秋珊莎锦黛青倩婷姣婉娴瑾颖露瑶怡婵雁蓓纨仪荷丹蓉眉君琴蕊薇菁梦岚苑婕馨瑗琰韵融园艺咏卿聪澜纯毓悦昭冰爽琬茗羽希宁欣飘育滢馥筠柔竹霭凝晓欢霄枫芸菲寒伊亚宜可姬舒影荔枝思丽";
    let female_names_len = female_names.chars().count();
    let male_names = "伟刚勇毅俊峰强军平保东文辉力明永健世广志义兴良海山仁波宁贵福生龙元全国胜学祥才发武新利清飞彬富顺信子杰涛昌成康星光天达安岩中茂进林有坚和彪博诚先敬震振壮会思群豪心邦承乐绍功松善厚庆磊民友裕河哲江超浩亮政谦亨奇固之轮翰朗伯宏言若鸣朋斌梁栋维启克伦翔旭鹏泽晨辰士以建家致树炎德行时泰盛雄琛钧冠策腾楠榕风航弘";
    let male_names_len = male_names.chars().count();
    let name_length = rand::thread_rng().gen_range(1..3) as usize;
    let is_male = rand::random::<u8>() % 2 == 0;
    if is_male {
        for _ in 0..name_length {
            result.push(
                male_names
                    .chars()
                    .nth(rand::thread_rng().gen_range(0..male_names_len))
                    .unwrap(),
            )
        }
    } else {
        for _ in 0..name_length {
            result.push(
                female_names
                    .chars()
                    .nth(rand::thread_rng().gen_range(0..female_names_len))
                    .unwrap(),
            )
        }
    };
    result
}
