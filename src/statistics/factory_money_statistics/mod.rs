use std::{collections::HashMap, path::PathBuf};

use plotters::{
    chart::ChartBuilder,
    prelude::{IntoDrawingArea, PathElement, SVGBackend},
    series::LineSeries,
    style::{IntoFont, BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, TRANSPARENT, YELLOW},
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
            .map(|(time, money)| (time.into_hours() as f64, f64::from(*money)));
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
        let time_margin = (max_time - min_time) / 20.0;
        let money_margin = (max_money - min_money) / 20.0;

        let root = SVGBackend::new(&self.output_file, (640, 480)).into_drawing_area();
        root.fill(&TRANSPARENT).unwrap();

        let styles = [RED, GREEN, BLUE, BLACK, CYAN, MAGENTA, YELLOW];
        let mut chart = ChartBuilder::on(&root)
            .caption("Factory Money Over Time", ("sans-serif", 24).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                min_time - time_margin..max_time + time_margin,
                min_money - money_margin..max_time + money_margin,
            )
            .unwrap();
        chart.configure_mesh().draw().unwrap();

        for ((factory_id, series), style) in self.money_time_series.iter().zip(&styles) {
            chart
                .draw_series(LineSeries::new(
                    series
                        .iter()
                        .map(|(time, money)| (time.into_hours() as f64, f64::from(*money))),
                    style,
                ))
                .unwrap()
                .label(format!("Factory {factory_id}"))
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], *style));
        }
    }
}
