use crate::cam::model::base::LoanType;

pub fn get_repay_type_nav(repay_type:String) -> Vec<String>{
    if repay_type == LoanType::MonthlyRepayInterest.to_string(){
        return vec![
            "定期付息计划".to_string(),
            "最新质押物信息".to_string(),
            "利息变动记录".to_string(),
            "质押信息变动记录".to_string(),
            "本金变动记录".to_string(),
            "期限调整记录".to_string(),
            "管理费记录".to_string(),
            "利息计提记录".to_string(),
            "资金变动记录".to_string(),
            "业务日志".to_string(),
            "附件".to_string(),
        ]
    }

    if repay_type == LoanType::OneTimeRepay.to_string(){
        return vec![
            "最新质押物信息".to_string(),
            "利息变动记录".to_string(),
            "质押信息变动记录".to_string(),
            "本金变动记录".to_string(),
            "期限调整记录".to_string(),
            "管理费记录".to_string(),
            "利息计提记录".to_string(),
            "资金变动记录".to_string(),
            "业务日志".to_string(),
            "附件".to_string(),
        ]
    }

    if repay_type == LoanType::MatchingService.to_string() || repay_type == LoanType::MatchingRepayment.to_string(){
        return vec![
            "还本付息计划".to_string(),
            "最新质押物信息".to_string(),
            "还本付息记录".to_string(),
            "质押信息变动记录".to_string(),
            "管理费记录".to_string(),
            "利息计提记录".to_string(),
            "资金变动记录".to_string(),
            "业务日志".to_string(),
            "附件".to_string(),
        ]
    }

    if repay_type == LoanType::PeriodCompounding.to_string(){
        return vec![
            "复利计划".to_string(),
            "最新质押物信息".to_string(),
            "利息变动记录".to_string(),
            "质押信息变动记录".to_string(),
            "本金变动记录".to_string(),
            "期限调整记录".to_string(),
            "管理费记录".to_string(),
            "利息计提记录".to_string(),
            "利率调整记录".to_string(),
            "资金变动记录".to_string(),
            "业务日志".to_string(),
            "附件".to_string(),
        ]
    }

    return vec![]
}

#[test]
fn test_get_repay_type_nav(){
    let result = get_repay_type_nav("".to_string());
    eprintln!("{:?}",result);

    let result = get_repay_type_nav("monthly-repay-interest".to_string());
    eprintln!("{:?}",result)
}

pub fn get_repay_type_nav_index(nav: Vec<String>,current:String) -> u32{
    let index = nav.iter().position(|item| item == &current).unwrap();
    return index as u32
}