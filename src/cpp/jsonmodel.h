// SPDX-License-Identifier: MIT

#pragma once

#include <QStandardItemModel>
#include <QQmlEngine>
#include <qtmetamacros.h>


/*
  Simple model that shows JSON data in a tree or list QStandardItemModel.
  Use for quick prototyping, not for production, as performance might not scale.
*/
class JsonModel : public QStandardItemModel
{
    Q_OBJECT
    QML_ELEMENT
    Q_PROPERTY(QString jsonText READ jsonText WRITE setJsonText NOTIFY jsonTextChanged)
    Q_PROPERTY(int count READ count NOTIFY countChanged)
public:
    explicit JsonModel(QObject *parent = nullptr);
    ~JsonModel() override;

    void setJsonText(const QString &text);
    QString jsonText() const;
    int count() const;

Q_SIGNALS:
    void jsonTextChanged();
    void countChanged();

protected:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

private:
    void addRow(const QJsonObject &row, QStandardItem *parent);
    QString m_jsonText;
    QStringList m_headerTitles;
};
