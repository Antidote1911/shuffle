#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QMessageBox>
#include <QTimer>
#include <QClipboard>
#include <QMessageBox>
#include <QRandomGenerator>
#include <QVector>

#include "adapter.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    ui->lb_status->setVisible(false);
    ui->cb_psnumbers->setChecked(true);
    ui->cb_psuppercase->setChecked(true);
    onGenerateClick();

    connect(ui->btn_generate, &QPushButton::clicked, this, &MainWindow::onGenerateClick);
    connect(ui->btn_copy, &QPushButton::clicked, this, &MainWindow::onCopyClick);
    connect(ui->optionsButtons, &QButtonGroup::buttonClicked, this, &MainWindow::onGenerateClick);
    connect(ui->horizontalSlider, &QSlider::sliderMoved, this, &MainWindow::onGenerateClick);


}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::onGenerateClick()
{
    const int  pwdLen = ui->horizontalSlider->sliderPosition();
    const bool lowerCase = ui->cb_pslowercase->isChecked();
    const bool upperCase = ui->cb_psuppercase->isChecked();
    const bool symbols = ui->cb_pssymbols->isChecked();
    const bool numbers = ui->cb_psnumbers->isChecked();

    if (!lowerCase && !upperCase && !symbols && !numbers)
    {
        ui->btn_generate->setDisabled(true);
        ui->horizontalSlider->setDisabled(true);
        ui->lb_status->setText("Select one or more groups");
        ui->lb_status->setVisible(true);
        ui->btn_copy->setDisabled(true);

    }else{
        ui->btn_generate->setDisabled(false);
        ui->horizontalSlider->setDisabled(false);
        ui->lb_status->setVisible(false);
        ui->btn_copy->setDisabled(false);
        auto test = get_random(pwdLen,upperCase,lowerCase,numbers,symbols);
        ui->passwordEdit->clear();
        ui->passwordEdit->setText(test);
    }
}


void MainWindow::onCopyClick()
{
    //copy to system clipboard only if there is a password set
    const QString pwd = ui->passwordEdit->text();
    if (pwd.length() >= 8)
    {
        QGuiApplication::clipboard()->setText(pwd);
        //show the label "copied to clipboard" for 2 seconds
        if (!ui->lb_status->isVisible())
        {
            ui->lb_status->setText("Password Copied to Clipboard");
            ui->lb_status->setVisible(true);
            QTimer::singleShot(2000, this, [this](){ ui->lb_status->setVisible(false); });
        }
    }
}

