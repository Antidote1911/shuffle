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
    ui->pushNumbers->setChecked(true);
    ui->pushUpper->setChecked(true);
    onGenerateClick();

    connect(ui->btn_generate, &QPushButton::clicked, this, &MainWindow::onGenerateClick);
    connect(ui->btn_copy, &QPushButton::clicked, this, &MainWindow::onCopyClick);
    connect(ui->optionsButtons, &QButtonGroup::buttonClicked, this, &MainWindow::onGenerateClick);
    connect(ui->horizontalSlider, &QSlider::sliderMoved, this, &MainWindow::onGenerateClick);

    connect(ui->horizontalSlider, SIGNAL(valueChanged(int)), SLOT(passwordLengthChanged(int)));
    connect(ui->spinBox, SIGNAL(valueChanged(int)), SLOT(passwordLengthChanged(int)));


}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::passwordLengthChanged(int length)
{
    ui->spinBox->blockSignals(true);
    ui->horizontalSlider->blockSignals(true);

    ui->spinBox->setValue(length);
    ui->horizontalSlider->setValue(length);

    ui->spinBox->blockSignals(false);
    ui->horizontalSlider->blockSignals(false);

    onGenerateClick();
}

void MainWindow::onGenerateClick()
{
    const int  pwdLen = ui->horizontalSlider->sliderPosition();
    const bool lowerCase = ui->pushLower->isChecked();
    const bool upperCase = ui->pushUpper->isChecked();
    const bool symbols = ui->pushSymbols->isChecked();
    const bool numbers = ui->pushNumbers->isChecked();

    const char* testavoid = "";
    if (ui->checkAvoidLookLike->isChecked())
{
     testavoid="0Oo1Iil";
}




    if (!lowerCase && !upperCase && !symbols && !numbers)
    {
        ui->btn_generate->setDisabled(true);
        ui->horizontalSlider->setDisabled(true);
        ui->statusbar->showMessage("Select one or more groups",2000);
        ui->btn_copy->setDisabled(true);

    }else{
        ui->btn_generate->setDisabled(false);
        ui->horizontalSlider->setDisabled(false);
        ui->btn_copy->setDisabled(false);
        auto test = get_random(pwdLen,upperCase,lowerCase,numbers,symbols,testavoid);
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
        ui->statusbar->showMessage("Password Copied to Clipboard",2000);
    }
}

