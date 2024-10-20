use std::{collections::HashMap, path::PathBuf};

use log::debug;
use plotters::{
    chart::{ChartBuilder, SeriesLabelPosition},
    prelude::{IntoDrawingArea, PathElement, SVGBackend},
    series::LineSeries,
    style::{IntoFont, BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, TRANSPARENT, WHITE, YELLOW},
};

use crate::{factory::FactoryId, money::Money, time::DateTime, world::World};

use super::Statistics;

#[derive(Debug)]
pub struct FactoryMoneyStatistics {
    output_file: PathBuf,
    money_time_series: HashMap<FactoryId, Vec<(DateTime, Money)>>,
}

impl FactoryMoneyStatistics {
    pub fn new(output_file: impl Into<PathBuf>) -> Self {
        Self {
            output_file: output_file.into(),
            money_time_series: Default::default(),
        }
    }
}

impl Statistics for FactoryMoneyStatistics {
    fn collect(&mut self, world: &World) {
        for (factory_id, factory) in world.factories() {
            let entry = (world.time(), factory.money());
            if let Some(money_time_series) = self.money_time_series.get_mut(&factory_id) {
                money_time_series.push(entry);
            } else {
                self.money_time_series.insert(factory_id, vec![entry]);
            }
        }
    }

    fn finalise(&self) {
        let mut iter = self
            .money_time_series
            .values()
            .flat_map(|money_time_series| money_time_series.iter())
            .copied();
        //.map(|(time, money)| (time.into_hours() as f64, f64::from(*money)));
        let (first_time, first_money) = iter.next().unwrap();
        let (min_time, max_time, min_money, max_money) = iter.fold(
            (first_time, first_time, first_money, first_money),
            |(min_time, max_time, min_money, max_money), (time, money)| {
                (
                    min_time.min(time),
                    max_time.max(time),
                    min_money.min(money),
                    max_money.max(money),
                )
            },
        );
        let time_margin = (max_time - min_time) / 20;
        let money_margin = (max_money - min_money) / 20;
        let chart_min_time = min_time.saturating_sub(time_margin);
        let chart_max_time = max_time + time_margin;
        let chart_min_money = min_money.saturating_sub(money_margin);
        let chart_max_money = max_money + money_margin;

        debug!("Drawing factory money statistics in area x: {chart_min_time}..{chart_max_time}; y: {chart_min_money}..{chart_max_money}");

        let root = SVGBackend::new(&self.output_file, (640, 480)).into_drawing_area();
        root.fill(&TRANSPARENT).unwrap();

        let styles = [RED, GREEN, BLUE, BLACK, CYAN, MAGENTA, YELLOW];
        let mut chart = ChartBuilder::on(&root)
            .caption("Factory Money Over Time", ("sans-serif", 24).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(50)
            .build_cartesian_2d(
                chart_min_time.into_hours()..chart_max_time.into_hours(),
                chart_min_money.raw()..chart_max_money.raw(),
            )
            .unwrap();
        chart
            .configure_mesh()
            .y_label_formatter(&format_money)
            .draw()
            .unwrap();

        for ((factory_id, series), style) in self.money_time_series.iter().zip(&styles) {
            chart
                .draw_series(LineSeries::new(
                    series
                        .iter()
                        .map(|(time, money)| (time.into_hours(), money.raw())),
                    style,
                ))
                .unwrap()
                .label(format!("Factory {factory_id}"))
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], *style));
        }

        chart
            .configure_series_labels()
            .background_style(WHITE)
            .border_style(BLACK)
            .position(SeriesLabelPosition::LowerRight)
            .draw()
            .unwrap();
    }
}

fn format_money(money: &u64) -> String {
    let money = *money as f64;
    if money < 1e3 {
        format!("{money}€")
    } else if money < 1e4 {
        format!("{:.2}k€", money / 1e3)
    } else if money < 1e5 {
        format!("{:.1}k€", money / 1e3)
    } else if money < 1e6 {
        format!("{:.0}k€", money / 1e3)
    } else if money < 1e7 {
        format!("{:.2}M€", money / 1e6)
    } else if money < 1e8 {
        format!("{:.1}M€", money / 1e6)
    } else if money < 1e9 {
        format!("{:.0}M€", money / 1e6)
    } else if money < 1e10 {
        format!("{:.2}G€", money / 1e9)
    } else if money < 1e11 {
        format!("{:.1}G€", money / 1e9)
    } else if money < 1e12 {
        format!("{:.0}G€", money / 1e9)
    } else if money < 1e13 {
        format!("{:.2}T€", money / 1e12)
    } else if money < 1e14 {
        format!("{:.1}T€", money / 1e12)
    } else {
        format!("{:.0}T€", money / 1e12)
    }
}
